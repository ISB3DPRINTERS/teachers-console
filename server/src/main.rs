#![allow(unused_must_use, unused_assignments, clippy::needless_return)]

// #[cfg(not(debug_assertions))]
pub mod client;

pub mod auth;
pub mod config;
pub mod db;
pub mod errors;
pub mod logging;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod state;
pub mod util;

use axum::{middleware::from_fn, Router, Server};
use dotenvy::dotenv;
use hyper::Body;
use log::info;
use std::{env, error::Error, net::SocketAddr};

use crate::{
    config::get_config, db::connect_db, middleware::logger::logging_middleware, state::AppState,
    util::parse_ip,
};

#[cfg(not(debug_assertions))]
use client::run_client_node;

#[cfg(not(debug_assertions))]
use routes::client_handler;

#[cfg(debug_assertions)]
use routes::handle_error;

#[cfg(debug_assertions)]
use log::warn;

#[cfg(not(debug_assertions))]
async fn _run_client() {
    info!("Starting client...");

    tokio::spawn(async {
        run_client_node().await;
    });
}

#[cfg(debug_assertions)]
async fn _run_client() {
    warn!("Not starting client, is a debug build.");
}

#[cfg(not(debug_assertions))]
fn _register_client_handler(router: Router<AppState, Body>) -> Router<AppState, Body> {
    return router.fallback(client_handler);
}

#[cfg(debug_assertions)]
fn _register_client_handler(router: Router<AppState, Body>) -> Router<AppState, Body> {
    return router.fallback(handle_error);
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    info!(
        "==== STARTUP: {} version {} ====",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("Connecting to database...");

    let config = get_config().await;
    let db = connect_db(&config.database_url).await?;

    info!("Connected!");

    let ip = parse_ip(config.clone().host);
    let address = SocketAddr::from((ip, config.clone().port));
    let server = Server::bind(&address);

    let router = Router::new();

    let router = routes::register(router);
    let router = _register_client_handler(router);
    let router = router.layer(from_fn(logging_middleware));

    let router = router.with_state(AppState::new(db, config));

    let service = router.into_make_service_with_connect_info::<SocketAddr>();
    let app = server.serve(service);

    _run_client().await;

    info!("Listening on {}", address);

    app.await.map_err(|v| v.into())
}

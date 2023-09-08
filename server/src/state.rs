use std::sync::{Arc, Mutex};

use diesel::PgConnection;
use hyper::{client::HttpConnector, Body, Client};

use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<PgConnection>>,
    pub config: AppConfig,
    pub client: Client<HttpConnector, Body>,
}

impl AppState {
    pub fn new(db: PgConnection, config: AppConfig) -> Self {
        return Self {
            db: Arc::new(Mutex::new(db)),
            config,
            client: Client::new(),
        };
    }
}

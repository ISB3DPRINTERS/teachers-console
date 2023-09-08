use axum::{http::Request, middleware::Next, response::Response};

use chrono::Utc;
use log::info;

use crate::logging::config::{BackgroundColors, Colors, ForegroundColors};

pub async fn logging_middleware<B>(req: Request<B>, next: Next<B>) -> Response {
    let time_start = Utc::now().time();

    let method = &req.method().clone();
    let uri = &req.uri().clone();

    let res = next.run(req).await;

    let now = Utc::now().time();

    let elapsed = now - time_start;

    info!(
        "{} {} {} {}{}{}{} {} ({} ms)",
        BackgroundColors::Blue,
        method.as_str(),
        Colors::Reset,
        ForegroundColors::Magenta,
        Colors::Bold,
        uri.path(),
        Colors::Reset,
        res.status(),
        elapsed.num_milliseconds()
    );

    return res;
}

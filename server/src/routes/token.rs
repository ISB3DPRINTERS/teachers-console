use crate::{
    auth::{data::LoginInfo, login::login},
    errors::conflict::BasicResponseError,
    state::AppState,
};

use axum::{debug_handler, extract::State, response::Response, Json};
use diesel::PgConnection;
use http::StatusCode;

#[debug_handler]
pub async fn get_token(
    State(state): State<AppState>,
    Json(user_info): Json<LoginInfo>,
) -> Result<Response<String>, Response<String>> {
    let resp = login(
        &mut state.db.lock().unwrap() as &mut PgConnection,
        &user_info,
    );

    if let Ok(user) = resp {
        return Ok(Response::new(user.token.unwrap()));
    }

    Err(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(
            serde_json::to_string(&BasicResponseError {
                code: 400,
                message: String::from("Invalid user!"),
            })
            .unwrap(),
        )
        .unwrap())
}

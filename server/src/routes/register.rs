use axum::{
    debug_handler,
    extract::State,
    http::{status::StatusCode, Response},
    Json,
};

use crate::{
    auth::register::create_user,
    errors::conflict::BasicResponseError,
    models::user::{CredentiallessUser, UserRegistrationInfo},
    state::AppState,
};

#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(info): Json<UserRegistrationInfo>,
) -> Result<Response<String>, Response<String>> {
    let resp = create_user(&mut state.db.lock().unwrap(), &info);

    if let Ok(user) = resp {
        return Ok(Response::new(
            serde_json::to_string(&CredentiallessUser::from(user)).unwrap(),
        ));
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

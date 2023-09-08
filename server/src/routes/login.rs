use axum::{debug_handler, extract::State, http::Response, response::Result, Json};

use diesel::PgConnection;
use http::StatusCode;

use crate::{
    auth::{
        data::{LoginInfo, TokenInfo},
        login::{login, login_token},
    },
    errors::conflict::BasicResponseError,
    models::user::{CredentiallessUser, User},
    state::AppState,
};

#[debug_handler]
pub async fn auth_login(
    State(state): State<AppState>,
    Json(user_info): Json<LoginInfo>,
) -> Result<Response<String>, Response<String>> {
    let resp = login(
        &mut state.db.lock().unwrap() as &mut PgConnection,
        &user_info,
    );

    if let Ok(user) = resp {
        return Ok(Response::new(
            serde_json::to_string(&CredentiallessUser::from(User::from(user))).unwrap(),
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

#[debug_handler]
pub async fn token_login(
    State(state): State<AppState>,
    Json(token_info): Json<TokenInfo>,
) -> Result<Response<String>, Response<String>> {
    let resp = login_token(
        &mut state.db.lock().unwrap() as &mut PgConnection,
        &token_info,
    );

    if let Ok(user) = resp {
        return Ok(Response::new(
            serde_json::to_string(&CredentiallessUser::from(User::from(user))).unwrap(),
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

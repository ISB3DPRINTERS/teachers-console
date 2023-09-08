use axum::{
    debug_handler,
    extract::{Path, State},
    response::Response,
};

use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use http::StatusCode;

use crate::{
    errors::conflict::BasicResponseError,
    models::user::{CredentiallessUser, User},
    schema::users::{id, table as users},
    state::AppState,
};

#[debug_handler]
pub async fn user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<String, Response<String>> {
    let user: Vec<User> = users
        .filter(id.eq(user_id))
        .limit(1)
        .select(User::as_select())
        .load(&mut state.db.lock().unwrap() as &mut PgConnection)
        .unwrap();

    if let Some(user) = user.get(0) {
        return Ok(serde_json::to_string(&CredentiallessUser::from(user.clone())).unwrap());
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

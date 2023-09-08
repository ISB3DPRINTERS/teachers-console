use crate::{
    auth::{data::LoginInfo, login::login},
    errors::conflict::BasicResponseError,
    state::AppState, schema::users::{table as users, token}, models::user::User,
};

use axum::{debug_handler, extract::State, response::Response, Json};
use diesel::{PgConnection, QueryDsl, ExpressionMethods, SelectableHelper, RunQueryDsl};
use http::StatusCode;
use rand::{distributions::{Alphanumeric, DistString}, thread_rng};

#[debug_handler]
pub async fn get_token(
    State(state): State<AppState>,
    Json(user_info): Json<LoginInfo>,
) -> Result<Response<String>, Response<String>> {
    let resp = login(
        &mut state.db.lock().unwrap() as &mut PgConnection,
        &user_info,
    );

    if let Ok(mut user) = resp {
        if user.token.is_none() {
            user.token = Some(Alphanumeric.sample_string(&mut thread_rng(), 128));

            diesel::update(users.find(user.id))
                .set(token.eq(user.token.clone()))
                .returning(User::as_returning())
                .get_result(&mut state.db.lock().unwrap() as &mut PgConnection)
                .unwrap();
        }

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

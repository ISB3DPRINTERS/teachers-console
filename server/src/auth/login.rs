use std::error::Error;

use bcrypt::verify;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};

use crate::{
    errors::db::DbError,
    models::user::IdentifiedUser,
    schema::users::{table as users, token, username},
};

use super::data::{LoginInfo, TokenInfo};

pub fn login(db: &mut PgConnection, login_info: &LoginInfo) -> Result<IdentifiedUser, Box<dyn Error>> {
    let user = users
        .filter(username.eq(login_info.username.clone()))
        .limit(1)
        .select(IdentifiedUser::as_select())
        .load(db)?;

    let user: Option<IdentifiedUser> = user.get(0).cloned();

    if let Some(user) = user {
        if verify(login_info.password.clone(), &user.password).unwrap() {
            return Ok(user);
        }
    }

    Err(Box::new(DbError::NotFound))
}

pub fn login_token(db: &mut PgConnection, token_info: &TokenInfo) -> Result<IdentifiedUser, Box<dyn Error>> {
    let user = users
        .filter(token.eq(token_info.token.clone()))
        .limit(1)
        .select(IdentifiedUser::as_select())
        .load(db)?;

    let user = user.get(0);

    if let Some(user) = user {
        return Ok(user.clone());
    }

    Err(Box::new(DbError::NotFound))
}

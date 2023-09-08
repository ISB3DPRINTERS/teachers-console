use std::error::Error;

use diesel::{insert_into, PgConnection, RunQueryDsl, SelectableHelper};

use crate::{
    models::user::{User, UserRegistrationInfo},
    schema::users,
};

pub fn create_user(
    db: &mut PgConnection,
    info: &UserRegistrationInfo,
) -> Result<User, Box<dyn Error>> {
    let user = User::new(info.clone());
    let user = insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(db)?;

    Ok(user.clone())
}

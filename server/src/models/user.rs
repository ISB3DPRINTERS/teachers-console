use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::util::hash::hash_password;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentiallessUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistrationInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(data: UserRegistrationInfo) -> Self {
        Self {
            first_name: data.first_name,
            last_name: data.last_name,
            email: data.email,
            username: data.username,
            password: hash_password(data.password.as_str()),
            token: None,
        }
    }

    pub const COLLECTION_NAME: &'static str = "users";
}

impl From<User> for CredentiallessUser {
    fn from(value: User) -> Self {
        Self {
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            username: value.username,
        }
    }
}

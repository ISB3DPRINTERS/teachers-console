use derive_more::{Display, Error};

#[derive(Debug, Clone, Display, Error)]
pub enum DbError {
    NotFound,
}

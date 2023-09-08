use std::error::Error;

use diesel::{Connection, PgConnection};

pub async fn connect_db(database_url: &str) -> Result<PgConnection, Box<dyn Error>> {
    let db = PgConnection::establish(database_url)?;

    Ok(db)
}

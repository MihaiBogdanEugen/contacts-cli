use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

const DATABASE_URL_KEY: &str = "DATABASE_URL";

fn get_db_connection() -> Result<PgConnection, String> {
    if dotenv().is_err() {
        return Err("Cannot load .env file".to_string());
    }

    let database_url: String = match env::var(DATABASE_URL_KEY) {
        Ok(x) => x,
        Err(_) => return Err(format!("Missing {} env. variable", DATABASE_URL_KEY)),
    };

    let db_connection = match PgConnection::establish(&database_url) {
        Ok(x) => x,
        Err(_) => return Err(format!("Cannot connect to {}", database_url)),
    };

    return Ok(db_connection);
}

mod schema;
mod models;

use diesel::prelude::*;
use std::env;
use dotenv::dotenv;
use anyhow::{Result, bail};
use self::models::User;
use diesel::insert_into;

fn connect(database_url: String) -> Result<MysqlConnection> {
    let connection = MysqlConnection::establish(&database_url);

    if let Err(e) = connection {
        bail!("Error: {}", e);
    }

    Ok(connection?)
}

fn main() {
    use self::schema::user::dsl::*;

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let connection = &mut connect(database_url).unwrap();
}


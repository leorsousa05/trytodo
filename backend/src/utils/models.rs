use std::env;

use anyhow::{bail, Result};
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};
use super::schema::{ todo_item, user };

pub fn connect() -> Result<MysqlConnection> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set!");

    let connection = MysqlConnection::establish(&database_url);

    if let Err(e) = connection {
        bail!("Error: {}", e);
    }

    Ok(connection?)
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

#[derive(Insertable, Serialize, Deserialize, Default, Debug)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub name: String,
    pub password: String,
    pub email: String
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = todo_item)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: Option<i32>
}

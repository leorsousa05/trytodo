use std::env;

use anyhow::{bail, Result};
use axum::Json;
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerResponse<T> {
    Data(T),
    Error(ResponseError)
}

impl<T> ServerResponse<T> {
    pub fn data(data: T) -> Json<Self> {
        Json(Self::Data(data))
    }

    pub fn error<E: AsRef<str>>(error: E) -> Json<Self> {
        Json(ServerResponse::Error(ResponseError::new(error)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct ResponseError {
    pub message: String
}

impl ResponseError {
    pub fn new<T: AsRef<str>>(message: T) -> Self {
        Self {
            message: message.as_ref().to_owned()
        }
    }
}



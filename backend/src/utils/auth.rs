use jsonwebtoken::{
    Header, 
    EncodingKey, 
    encode, 
    DecodingKey, 
    Validation, 
    decode
};
use serde_derive::{Serialize, Deserialize};

use crate::database::SECRET;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64
}

impl Claims {
    pub fn new() -> Self {
        let sub = "User".into();
        let exp = (chrono::Utc::now() + chrono::Duration::hours(2)).timestamp();

        Self {
            sub,
            exp
        }
    }
}

pub fn sign() -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        &Claims::new(),
        &EncodingKey::from_secret(SECRET.as_bytes())
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode(
        token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::default(),
    ).map(|data| data.claims)
}

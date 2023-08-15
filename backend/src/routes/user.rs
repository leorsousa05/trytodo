use axum::http::StatusCode;
use axum::Json;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::{ QueryDsl, ExpressionMethods, SelectableHelper, RunQueryDsl };
use diesel::insert_into;
use serde_derive::Deserialize;
use crate::utils::auth::{create_token, verify_token};
use crate::models::user::{ User, CreateUser, AuthUser };
use crate::utils::schema::user::dsl::*;
use crate::database::DB_CONNECTION;
use crate::utils::server::ServerResponse;

pub async fn select_users() -> (StatusCode, Json<ServerResponse<Vec<User>>>) {
    let mut db = DB_CONNECTION.lock().unwrap();

    let users = user
        .select(User::as_select())
        .load(&mut *db);

    if let Err(_e) = users {
        return (StatusCode::INTERNAL_SERVER_ERROR, ServerResponse::error("Internal Server Error"));
    }

    (StatusCode::OK, ServerResponse::data(users.unwrap()))
}

pub async fn create_user(Json(body): Json<CreateUser>) -> (StatusCode, Json<ServerResponse<String>>) {
    let db = &mut *DB_CONNECTION.lock().unwrap();

    let hashed_password = hash(body.password, DEFAULT_COST).unwrap();

    let users = insert_into(user)
        .values(&CreateUser {
            name: body.name,
            password: hashed_password,
            email: body.email
        })
    .execute(db);

    if let Err(_e) = users {
        return (StatusCode::INTERNAL_SERVER_ERROR, ServerResponse::error("Failed to Create User"))
    }

    (StatusCode::OK, ServerResponse::data("User Created!".into()))
}

pub async fn authenticate(Json(body): Json<AuthUser>) -> (StatusCode, Json<ServerResponse<String>>) {

    let db = &mut *DB_CONNECTION.lock().unwrap(); 

    let auth_user_data: Result<User, diesel::result::Error> = user
        .filter(email.eq(&body.email))
        .first::<User>(db);

    if let Err(_e) = auth_user_data {
        return (StatusCode::BAD_REQUEST, ServerResponse::error("Email is wrong or User does not exist!"));
    }

    let verify = verify(body.password, &auth_user_data.unwrap().password).unwrap();

    if !verify {
        return (StatusCode::BAD_REQUEST, ServerResponse::error("Password is Wrong!"));
    }

    let token = create_token().unwrap();

    (StatusCode::OK, ServerResponse::data(token))
}

#[derive(Deserialize, Debug)]
pub struct TokenVerification {
    pub token: String
}

pub async fn token_verification(Json(body): Json<TokenVerification>) -> (StatusCode, Json<ServerResponse<String>>) {
    let verified_tokens = verify_token(&body.token);

    if let Err(_e) = verified_tokens {
        return (StatusCode::BAD_REQUEST, ServerResponse::error("Token not valid!"));
    }

    (StatusCode::OK, ServerResponse::data("Valid Token!".into()))
}

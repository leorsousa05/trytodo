use axum::Router;
use axum::routing::{ get, post };

use crate::routes::user::{select_users, create_user, authenticate, token_verification};

pub fn app() -> Router {
    Router::new()
        .route("/users", get(select_users))
        .route("/signin", post(create_user))
        .route("/auth", post(authenticate))
        .route("/auth/verification", post(token_verification))
}



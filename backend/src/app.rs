use axum::routing::{get, post};
use axum::Router;

use crate::routes::todo::{create_todo, select_todo, select_user_todos};
use crate::routes::user::{authenticate, create_user, select_users, token_verification};

pub fn app() -> Router {
    Router::new()
        .route("/users", get(select_users))
        .route("/signin", post(create_user))
        .route("/auth", post(authenticate))
        .route("/auth/verification", post(token_verification))
        .route("/todo/create", post(create_todo))
        .route("/todo/user", post(select_user_todos))
        .route("/todo/select", post(select_todo))
}

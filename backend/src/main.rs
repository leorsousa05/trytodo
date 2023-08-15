use app::app;

mod utils;
mod tests;
mod routes;
mod database;
mod app;
mod models;

#[tokio::main]
async fn main() {
    let app = app();

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


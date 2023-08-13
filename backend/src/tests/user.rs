use axum::{http::{ Request, Method }, body::Body};
use serde_json::Value;

#[allow(dead_code)]
pub fn api_request(method: Method, body: Value, uri: String) -> Request<Body> {
    Request::builder()
        .header("content-type", "application/json")
        .method(method)
        .uri(uri)
        .body(Body::from(body.to_string()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use axum::http::Method;

    use super::api_request;
    use reqwest::StatusCode;
    use serde_json::json;
    use tower::ServiceExt;

    use crate::app::app;

    #[ignore = "Not being used"]
    #[tokio::test]
    async fn user_creation() {
        let app = app();

        let new_user = json!({
            "name": "John doe",
            "password": "123",
            "email": "joaozinho@gmail.com"
        });

        let request = api_request(Method::POST, new_user, "/signin".into());

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn authentication() {
        let app = app();

        let user_data = json!({
            "email": "leorsousa05@gmail.com",
            "password": "190405lrs"
        });

        let request = api_request(Method::POST, user_data, "/auth".into());

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK)
    }

}


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
    async fn user_authentication() {
        let app = app();

        let success_user_data = json!({
            "email": "leorsousa05@gmail.com",
            "password": "190405lrs"
        });

        let fail_user_data = json!({
            "email": "error@gmail.com",
            "password": "Error"
        });

        let success_request = api_request(Method::POST, success_user_data, "/auth".into());
        let fail_request = api_request(Method::POST, fail_user_data, "/auth".into());

        let success_response = app.clone().oneshot(success_request).await.unwrap();
        let fail_response = app.oneshot(fail_request).await.unwrap();

        assert_eq!(success_response.status(), StatusCode::OK);
        assert_eq!(fail_response.status(), StatusCode::BAD_REQUEST);
    }

}


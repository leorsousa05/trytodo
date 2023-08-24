#[cfg(test)]
pub mod tests {
    use reqwest::{Method, StatusCode};
    use serde_json::json;
    use tower::ServiceExt;

    use crate::{app::app, utils::server::api_request};

    #[ignore = "Row Creation!"]
    #[tokio::test]
    async fn todo_creation() {
        let app = app();

        let new_todo = json!({
            "title": "Test",
            "description": "Test",
            "user_id": 1
        });

        let bad_new_todo = json!({
            "title": "test",
            "description": "test",
            "user_id": -4
        });

        let request = api_request(Method::POST, new_todo.clone(), "/todo/create".to_string());
        let bad_request = api_request(
            Method::POST,
            bad_new_todo.clone(),
            "/todo/create".to_string(),
        );

        let response = app.clone().oneshot(request).await.unwrap();
        let bad_response = app.oneshot(bad_request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(bad_response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn select_todos() {
        let app = app();

        let request = api_request(Method::POST, json!({ "id": 1 }), "/todo/all".to_string());
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}

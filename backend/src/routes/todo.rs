use crate::database::DB_CONNECTION;
use crate::models::todo::{CreateTodoItem, SelectTodoItem, SelectUserTodoItems, TodoItem};
use crate::utils::schema::todo_item::dsl::*;
use crate::utils::server::ServerResponse;
use axum::{http::StatusCode, Json};
use diesel::{insert_into, ExpressionMethods, QueryDsl};
use diesel::{RunQueryDsl, SelectableHelper};

pub async fn select_user_todos(
    Json(body): Json<SelectUserTodoItems>,
) -> (StatusCode, Json<ServerResponse<Vec<TodoItem>>>) {
    let db = &mut *DB_CONNECTION.lock().unwrap();

    let todo_items = todo_item
        .select(TodoItem::as_select())
        .filter(user_id.eq(body.user_id))
        .load(db);

    if let Err(_e) = todo_items {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ServerResponse::error("Items not found!"),
        );
    }

    (StatusCode::OK, ServerResponse::data(todo_items.unwrap()))
}

pub async fn create_todo(
    Json(body): Json<CreateTodoItem>,
) -> (StatusCode, Json<ServerResponse<String>>) {
    let db = &mut *DB_CONNECTION.lock().unwrap();

    let todo_items = insert_into(todo_item)
        .values(&CreateTodoItem {
            title: body.title,
            description: body.description,
            user_id: body.user_id,
        })
        .execute(db);

    if let Err(_e) = todo_items {
        return (
            StatusCode::BAD_REQUEST,
            ServerResponse::error("Something happened, try again later!"),
        );
    }

    (
        StatusCode::OK,
        ServerResponse::data("Todo Created!".to_string()),
    )
}

pub async fn select_todo(
    Json(body): Json<SelectTodoItem>,
) -> (StatusCode, Json<ServerResponse<TodoItem>>) {
    let db = &mut *DB_CONNECTION.lock().unwrap();

    let todo_list_item = todo_item.filter(id.eq(&body.id)).first::<TodoItem>(db);

    if let Err(_e) = todo_list_item {
        return (
            StatusCode::BAD_REQUEST,
            ServerResponse::error("Item Don't Exist!"),
        );
    };

    (
        StatusCode::OK,
        ServerResponse::data(todo_list_item.unwrap()),
    )
}

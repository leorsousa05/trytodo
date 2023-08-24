use crate::utils::schema::todo_item;
use diesel::{Insertable, Queryable, Selectable};
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = todo_item)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = todo_item)]
pub struct CreateTodoItem {
    pub title: String,
    pub description: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = todo_item)]
pub struct SelectUserTodoItems {
    pub user_id: i32,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = todo_item)]
pub struct SelectTodoItem {
    pub id: i32,
}

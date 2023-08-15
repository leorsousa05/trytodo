use diesel::{Queryable, Selectable};
use crate::utils::schema::todo_item;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = todo_item)]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub user_id: Option<i32>
}

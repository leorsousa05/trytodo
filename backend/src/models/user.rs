use diesel::{Queryable, Selectable, Insertable};
use serde_derive::{Serialize, Deserialize};
use crate::utils::schema::user;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub email: String
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

#[derive(Insertable, Serialize, Deserialize, Default, Debug)]
#[diesel(table_name = user)]
pub struct CreateUser {
    pub name: String,
    pub password: String,
    pub email: String
}

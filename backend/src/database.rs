use std::sync::{Arc, Mutex};

use diesel::MysqlConnection;
use lazy_static::lazy_static; 
use crate::utils::models::connect;

lazy_static! {
    pub static ref DB_CONNECTION: Arc<Mutex<MysqlConnection>> = Arc::new(Mutex::new(connect().unwrap()));
    pub static ref SECRET: String  = "batatinhaquandonasceespalhaaramapelochama".into();
}

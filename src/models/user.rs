use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub(crate) id: i32,
    pub(crate) name: Option<String>,
    pub(crate) email: Option<String>,
    pub(crate) password: Option<String>,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct LoginResponse {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) email: String,
}

#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(sqlx::FromRow, Debug, Deserialize)]
pub struct RegisterRequest {
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) password_confirm: String,
}

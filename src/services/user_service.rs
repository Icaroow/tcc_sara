use sqlx::PgPool;
use crate::models::user::{User, CreateUser};

pub async fn get_users(pool: &PgPool) -> Vec<User> {
    sqlx::query_as::<_, User>(
        "SELECT id, name FROM users ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

pub async fn create_user(pool: &PgPool, payload: CreateUser) -> User {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name"
    )
    .bind(&payload.name)
    .fetch_one(pool)
    .await
    .expect("Erro ao criar usuário")
}

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Option<User> {
    sqlx::query_as::<_, User>(
        "SELECT id, name FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
}

pub async fn delete_user(pool: &PgPool, id: i32) -> bool {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .unwrap_or(false)
}
use axum::{Json, extract::{Path, State}};
use sqlx::PgPool;

use crate::models::user::{User, CreateUser};
use crate::services::user_service::{
    get_users,
    create_user,
    get_user_by_id,
    delete_user,
};

// GET /users
pub async fn get_users_handler(pool: PgPool) -> Json<Vec<User>> {
    let users = get_users(&pool).await;
    Json(users)
}

// POST /users
pub async fn create_user_handler(
    pool: PgPool,
    Json(payload): Json<CreateUser>,
) -> Json<User> {
    let user = create_user(&pool, payload).await;
    Json(user)
}

// GET /users/:id
pub async fn get_user_by_id_handler(
    Path(id): Path<i32>,
    pool: PgPool,
) -> Option<Json<User>> {
    get_user_by_id(&pool, id).await.map(Json)
}

// DELETE /users/:id
pub async fn delete_user_handler(
    Path(id): Path<i32>,
    pool: PgPool,
) -> String {
    let deleted = delete_user(&pool, id).await;

    if deleted {
        format!("Usuário {} deletado", id)
    } else {
        format!("Usuário {} não encontrado", id)
    }
}
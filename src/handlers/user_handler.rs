use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use crate::models::user::{User, CreateUser};
use crate::services::user_service::{
    get_users,
    create_user,
    get_user_by_id,
    delete_user,
};

// GET /users
pub async fn get_users_handler(
    State(pool): State<PgPool>,
) -> Json<Vec<User>> {
    let users = get_users(&pool).await;
    Json(users)
}

// POST /users
pub async fn create_user_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = create_user(&pool, payload).await;
    (StatusCode::CREATED, Json(user))
}

// GET /users/:id
pub async fn get_user_by_id_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match get_user_by_id(&pool, id).await {
        Some(user) => (StatusCode::OK, Json(Some(user))).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// DELETE /users/:id
pub async fn delete_user_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let deleted = delete_user(&pool, id).await;

    if deleted {
        (StatusCode::OK, format!("Usuário {} deletado", id))
    } else {
        (StatusCode::NOT_FOUND, format!("Usuário {} não encontrado", id))
    }
}
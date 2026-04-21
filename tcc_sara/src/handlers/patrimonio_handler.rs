use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use crate::models::patrimonio::{Patrimonio, CreatePatrimonio, UpdatePatrimonio};
use crate::services::patrimonio_service::{
    get_patrimonios,
    get_patrimonios_by_user,
    get_patrimonio_by_id,
    create_patrimonio,
    update_patrimonio,
    delete_patrimonio,
    get_total_valor_patrimonios,
};

// GET /patrimonios
pub async fn get_patrimonios_handler(
    State(pool): State<PgPool>,
) -> Json<Vec<Patrimonio>> {
    let patrimonios = get_patrimonios(&pool).await;
    Json(patrimonios)
}

// GET /patrimonios/user/:user_id
pub async fn get_patrimonios_by_user_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Json<Vec<Patrimonio>> {
    let patrimonios = get_patrimonios_by_user(&pool, user_id).await;
    Json(patrimonios)
}

// GET /patrimonios/:id
pub async fn get_patrimonio_by_id_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    match get_patrimonio_by_id(&pool, id).await {
        Some(patrimonio) => (StatusCode::OK, Json(Some(patrimonio))).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// POST /patrimonios
pub async fn create_patrimonio_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePatrimonio>,
) -> (StatusCode, Json<Patrimonio>) {
    let patrimonio = create_patrimonio(&pool, payload).await;
    (StatusCode::CREATED, Json(patrimonio))
}

// PUT /patrimonios/:id
pub async fn update_patrimonio_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePatrimonio>,
) -> impl IntoResponse {
    match update_patrimonio(&pool, id, payload).await {
        Some(patrimonio) => (StatusCode::OK, Json(Some(patrimonio))).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

// DELETE /patrimonios/:id
pub async fn delete_patrimonio_handler(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let deleted = delete_patrimonio(&pool, id).await;

    if deleted {
        (StatusCode::OK, format!("Patrimônio {} deletado", id)).into_response()
    } else {
        (StatusCode::NOT_FOUND, format!("Patrimônio {} não encontrado", id)).into_response()
    }
}

// GET /patrimonios/user/:user_id/total
pub async fn get_total_valor_handler(
    State(pool): State<PgPool>,
    Path(user_id): Path<i32>,
) -> Json<serde_json::Value> {
    let total = get_total_valor_patrimonios(&pool, user_id).await;
    Json(serde_json::json!({ "total": total }))
}
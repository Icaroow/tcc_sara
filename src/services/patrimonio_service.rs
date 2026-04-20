use sqlx::PgPool;
use crate::models::patrimonio::{Patrimonio, CreatePatrimonio, UpdatePatrimonio};

pub async fn get_patrimonios(pool: &PgPool) -> Vec<Patrimonio> {
    sqlx::query_as::<_, Patrimonio>(
        "SELECT id, descricao, numero_serie, valor, data_aquisicao, localizacao, status, user_id 
         FROM patrimonios ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

pub async fn get_patrimonios_by_user(pool: &PgPool, user_id: i32) -> Vec<Patrimonio> {
    sqlx::query_as::<_, Patrimonio>(
        "SELECT id, descricao, numero_serie, valor, data_aquisicao, localizacao, status, user_id 
         FROM patrimonios WHERE user_id = $1 ORDER BY id"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

pub async fn get_patrimonio_by_id(pool: &PgPool, id: i32) -> Option<Patrimonio> {
    sqlx::query_as::<_, Patrimonio>(
        "SELECT id, descricao, numero_serie, valor, data_aquisicao, localizacao, status, user_id 
         FROM patrimonios WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
}

pub async fn create_patrimonio(pool: &PgPool, payload: CreatePatrimonio) -> Patrimonio {
    sqlx::query_as::<_, Patrimonio>(
        "INSERT INTO patrimonios (descricao, numero_serie, valor, data_aquisicao, localizacao, status, user_id) 
         VALUES ($1, $2, $3, $4, $5, $6, $7) 
         RETURNING id, descricao, numero_serie, valor, data_aquisicao, localizacao, status, user_id"
    )
    .bind(&payload.descricao)
    .bind(&payload.numero_serie)
    .bind(payload.valor)
    .bind(&payload.data_aquisicao)
    .bind(&payload.localizacao)
    .bind("ativo")
    .bind(payload.user_id)
    .fetch_one(pool)
    .await
    .expect("Erro ao criar patrimônio")
}

pub async fn update_patrimonio(pool: &PgPool, id: i32, payload: UpdatePatrimonio) -> Option<Patrimonio> {
    let patrimonio = get_patrimonio_by_id(pool, id).await?;
    
    let descricao = payload.descricao.unwrap_or(patrimonio.descricao);
    let numero_serie = payload.numero_serie.unwrap_or(patrimonio.numero_serie);
    let valor = payload.valor.unwrap_or(patrimonio.valor);
    let localizacao = payload.localizacao.unwrap_or(patrimonio.localizacao);
    let status = payload.status.unwrap_or(patrimonio.status);

    sqlx::query_as::<_, Patrimonio>(
        "UPDATE patrimonios 
         SET descricao = $1, numero_serie = $2, valor = $3, localizacao = $4, status = $5 
         WHERE id = $6 
         RETURNING id, descricao, numero_serie, valor, data_aquisicao, localizacao, status, user_id"
    )
    .bind(descricao)
    .bind(numero_serie)
    .bind(valor)
    .bind(localizacao)
    .bind(status)
    .bind(id)
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
}

pub async fn delete_patrimonio(pool: &PgPool, id: i32) -> bool {
    sqlx::query("DELETE FROM patrimonios WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map(|r| r.rows_affected() > 0)
        .unwrap_or(false)
}

pub async fn get_total_valor_patrimonios(pool: &PgPool, user_id: i32) -> f64 {
    let result: (Option<f64>,) = sqlx::query_as(
        "SELECT SUM(valor) FROM patrimonios WHERE user_id = $1 AND status = 'ativo'"
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
    .unwrap_or((None,));
    
    result.0.unwrap_or(0.0)
}
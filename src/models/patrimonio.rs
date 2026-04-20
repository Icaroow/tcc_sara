use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone)]
pub struct Patrimonio {
    pub id: i32,
    pub descricao: String,
    pub numero_serie: String,
    pub valor: f64,
    pub data_aquisicao: String,
    pub localizacao: String,
    pub status: String, // ativo, inativo, descartado
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct CreatePatrimonio {
    pub descricao: String,
    pub numero_serie: String,
    pub valor: f64,
    pub data_aquisicao: String,
    pub localizacao: String,
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct UpdatePatrimonio {
    pub descricao: Option<String>,
    pub numero_serie: Option<String>,
    pub valor: Option<f64>,
    pub localizacao: Option<String>,
    pub status: Option<String>,
}
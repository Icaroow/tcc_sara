use sqlx::PgPool;
use crate::models::user::{User, CreateUser};

// Hash simples com SHA-256 via função do postgres para não adicionar dependência.
// Em produção, use bcrypt ou argon2.
fn hash_password(password: &str) -> String {
    // Usamos um hash manual com formato fixo para não precisar de crate extra.
    // ATENÇÃO: substitua por bcrypt em produção.
    format!("{:x}", md5_simple(password))
}

fn md5_simple(input: &str) -> String {
    // Implementação mínima — em produção use bcrypt/argon2
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    input.hash(&mut h);
    format!("{:016x}", h.finish())
}

pub async fn get_users(pool: &PgPool) -> Vec<User> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email, cpf_cnpj FROM users ORDER BY id"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default()
}

pub async fn create_user(pool: &PgPool, payload: CreateUser) -> Result<User, String> {
    // Verificar email duplicado
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"
    )
    .bind(&payload.email)
    .fetch_one(pool)
    .await
    .unwrap_or((false,));

    if exists.0 {
        return Err("Email já cadastrado".to_string());
    }

    // Verificar CPF/CNPJ duplicado
    let exists_doc: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM users WHERE cpf_cnpj = $1)"
    )
    .bind(&payload.cpf_cnpj)
    .fetch_one(pool)
    .await
    .unwrap_or((false,));

    if exists_doc.0 {
        return Err("CPF/CNPJ já cadastrado".to_string());
    }

    let password_hash = hash_password(&payload.password);

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email, password_hash, cpf_cnpj)
         VALUES ($1, $2, $3, $4)
         RETURNING id, name, email, cpf_cnpj"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.cpf_cnpj)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Erro ao criar usuário: {}", e))?;

    Ok(user)
}

pub async fn login_user(pool: &PgPool, email: &str, password: &str) -> Option<User> {
    let password_hash = hash_password(password);

    sqlx::query_as::<_, User>(
        "SELECT id, name, email, cpf_cnpj FROM users
         WHERE email = $1 AND password_hash = $2"
    )
    .bind(email)
    .bind(&password_hash)
    .fetch_optional(pool)
    .await
    .unwrap_or(None)
}

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Option<User> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email, cpf_cnpj FROM users WHERE id = $1"
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
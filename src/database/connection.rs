use sqlx::PgPool;

pub async fn connect_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL deve estar definida no .env");

    PgPool::connect(&database_url)
        .await
        .expect("Falha ao conectar ao banco de dados")
}
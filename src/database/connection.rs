use sqlx::PgPool;

pub async fn connect_db() -> PgPool {
    PgPool::connect("postgres://icaro:senhatop@localhost/meu_projeto")
        .await
        .unwrap()
}
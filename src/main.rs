mod routes;
mod handlers;
mod models;
mod services;
mod database;

use axum::Router;
use sqlx::PgPool;
use database::connection::connect_db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool: PgPool = connect_db().await;

    let app = Router::new()
        .merge(routes::user_routes::routes())
        .with_state(pool);

    println!("Servidor rodando em http://127.0.0.1:3000");

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
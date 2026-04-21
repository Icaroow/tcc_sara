mod routes;
mod handlers;
mod models;
mod services;
mod database;

use axum::Router;
use sqlx::PgPool;
use database::connection::connect_db;
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool: PgPool = connect_db().await;

    // CORS configurado para permitir requisições do frontend
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .merge(routes::user_routes::routes())
        .merge(routes::patrimonio_routes::routes())
        // Servir arquivos estáticos do frontend
        .nest_service("/", ServeDir::new("frontend"))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Servidor rodando em http://127.0.0.1:3000");
    println!("Frontend: http://127.0.0.1:3000/login.html");

    axum::serve(listener, app)
        .await
        .unwrap();
}
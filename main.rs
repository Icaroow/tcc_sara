mod routes;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(routes::user_routes::routes());

    println!("Servidor rodando em http://localhost:3000");

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(),
        app
    ).await.unwrap();
}
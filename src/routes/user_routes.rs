use axum::{Router, routing::{get, post, delete}};

use crate::handlers::user_handler::{
    get_users_handler,
    create_user_handler,
    get_user_by_id_handler,
    delete_user_handler,
};

pub fn routes() -> Router {
    Router::new()
        .route("/users", get(get_users_handler).post(create_user_handler))
        .route("/users/:id", get(get_user_by_id_handler).delete(delete_user_handler))
}
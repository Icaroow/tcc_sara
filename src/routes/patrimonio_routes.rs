use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::handlers::patrimonio_handler::{
    get_patrimonios_handler,
    get_patrimonios_by_user_handler,
    get_patrimonio_by_id_handler,
    create_patrimonio_handler,
    update_patrimonio_handler,
    delete_patrimonio_handler,
    get_total_valor_handler,
};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/patrimonios", get(get_patrimonios_handler).post(create_patrimonio_handler))
        .route("/patrimonios/:id", 
            get(get_patrimonio_by_id_handler)
            .put(update_patrimonio_handler)
            .delete(delete_patrimonio_handler)
        )
        .route("/patrimonios/user/:user_id", get(get_patrimonios_by_user_handler))
        .route("/patrimonios/user/:user_id/total", get(get_total_valor_handler))
}
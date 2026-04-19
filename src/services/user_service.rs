use sqlx::PgPool;
use crate::models::user::User;

pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Option<User> {
    let result = sqlx::query_as!(
        User,
        "SELECT id, name FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await
    .unwrap();

    result
}

pub async fn delete_user(pool: &PgPool, id: i32) -> bool {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        id
    )
    .execute(pool)
    .await
    .unwrap();

    result.rows_affected() > 0
}
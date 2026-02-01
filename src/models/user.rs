#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    user_id: uuid::Uuid,
    username: String,
    email: Option<String>,
}

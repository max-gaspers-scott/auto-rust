#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    user_id: uuid::Uuid,
    username: String,
    email: Option<String>,
    display_name: Option<String>,
    created_at: chrono::DateTime<chrono::Utc>,
}

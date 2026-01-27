#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Conversation {
    conversation_id: uuid::Uuid,
    name: Option<String>,
    is_group_chat: bool,
    created_at: chrono::DateTime<chrono::Utc>,
}

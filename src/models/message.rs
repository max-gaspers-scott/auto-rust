#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Message {
    message_id: uuid::Uuid,
    conversation_id: uuid::Uuid,
    sender_id: uuid::Uuid,
    content: String,
    sent_at: chrono::DateTime<chrono::Utc>,
    edited_at: Option<chrono::DateTime<chrono::Utc>>,
}

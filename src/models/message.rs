#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Message {
    message_id: uuid::Uuid,
    sender_id: uuid::Uuid,
    recipiant_id: uuid::Uuid,
    content: String,
    sent_at: chrono::DateTime<chrono::Utc>,
}

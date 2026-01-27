#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ConversationParticipant {
    participant_id: uuid::Uuid,
    conversation_id: uuid::Uuid,
    user_id: uuid::Uuid,
    joined_at: chrono::DateTime<chrono::Utc>,
}

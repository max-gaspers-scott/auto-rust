#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Host {
    host_id: uuid::Uuid,
    name: String,
    email: String,
    password_hash: String,
    zip_code: String,
}

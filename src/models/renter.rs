#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Renter {
    renter_id: uuid::Uuid,
    name: String,
    email: String,
    password_hash: String,
}

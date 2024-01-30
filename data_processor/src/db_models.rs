use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct VnasFetchRecord {
    pub id: i32,
    pub update_time: DateTime<Utc>,
    pub success: bool,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Artcc {
    pub id: String,
    pub last_updated: DateTime<Utc>,
}

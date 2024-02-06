use std::fmt::Debug;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Deserialize)]
pub struct ResponseBase {
    pub created_at: DateTime<Utc>,
    pub id: Thing,
    pub last_used: DateTime<Utc>,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Base {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
}

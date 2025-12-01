use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub name: String,
}



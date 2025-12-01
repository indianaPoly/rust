use crate::db::DbPool;
use crate::models::item::{CreateItemRequest, Item};

/// 아이템 전체 목록 조회
pub async fn get_items(pool: &DbPool) -> Result<Vec<Item>, sqlx::Error> {
    let items = sqlx::query_as::<_, Item>("SELECT id, name FROM items ORDER BY id")
        .fetch_all(pool)
        .await?;
    Ok(items)
}

/// 단일 아이템 조회
pub async fn get_item_by_id(pool: &DbPool, id: i64) -> Result<Option<Item>, sqlx::Error> {
    let item = sqlx::query_as::<_, Item>("SELECT id, name FROM items WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(item)
}

/// 아이템 생성
pub async fn create_item(pool: &DbPool, payload: CreateItemRequest) -> Result<Item, sqlx::Error> {
    let item = sqlx::query_as::<_, Item>(
        "INSERT INTO items (name) VALUES ($1) RETURNING id, name",
    )
    .bind(payload.name)
    .fetch_one(pool)
    .await?;

    Ok(item)
}



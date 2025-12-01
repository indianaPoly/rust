use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::{
    db::{item_repository, DbPool},
    error::AppError,
    models::item::CreateItemRequest,
};

/// 아이템 목록 조회
pub async fn list_items(State(pool): State<DbPool>) -> Result<impl IntoResponse, AppError> {
    let items = item_repository::get_items(&pool).await?;
    Ok((StatusCode::OK, Json(items)))
}

/// 단일 아이템 조회
pub async fn get_item(
    State(pool): State<DbPool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let item = item_repository::get_item_by_id(&pool, id).await?;

    match item {
        Some(item) => Ok((StatusCode::OK, Json(item)).into_response()),
        None => Err(AppError::NotFound(format!("Item({}) not found", id))),
    }
}

/// 아이템 생성
pub async fn create_item(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    let created = item_repository::create_item(&pool, payload).await?;
    Ok((StatusCode::CREATED, Json(created)))
}



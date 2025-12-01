use axum::{routing::get, Router};

use crate::{
    controllers::{health_controller, item_controller},
    db::DbPool,
};

/// 애플리케이션 루트 라우터를 생성합니다.
pub fn create_app(pool: DbPool) -> Router {
    Router::new()
        .route("/health", get(health_controller::health_check))
        .nest("/api/v1", api_router())
        .with_state(pool)
}

fn api_router() -> Router<DbPool> {
    Router::new()
        .route("/items", get(item_controller::list_items).post(item_controller::create_item))
        .route("/items/:id", get(item_controller::get_item))
}



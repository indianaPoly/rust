use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::health::HealthResponse;

pub async fn health_check() -> impl IntoResponse {
    let res = HealthResponse {
        status: "ok".into(),
    };
    (StatusCode::OK, Json(res))
}



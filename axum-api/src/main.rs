mod config;
mod controllers;
mod db;
mod error;
mod models;
mod routes;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use axum::serve;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::pool::create_pool;

#[tokio::main]
async fn main() {
    init_tracing();

    // 환경 변수에서 DB URL 로드 (예: postgres://user:password@localhost:5432/mydb)
    let database_url = config::database_url();

    // DB 커넥션 풀 생성
    let pool = match create_pool(&database_url).await {
        Ok(pool) => pool,
        Err(err) => {
            error!("failed to create DB pool: {}", err);
            return;
        }
    };

    // 라우터 구성 (의존성 주입: DB 풀)
    let app = routes::create_app(pool);

    // 서버 바인딩 주소
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server running at http://{}", addr);

    // TCP 리스너 생성
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind TCP listener");

    // 서버 시작
    if let Err(err) = serve(listener, app).await {
        error!("server error: {}", err);
    }
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "axum_api=debug,tower_http=debug,axum=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();
}



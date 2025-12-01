use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/// 애플리케이션 전역에서 사용할 Postgres 커넥션 풀 타입
pub type DbPool = Pool<Postgres>;

/// DATABASE_URL을 받아 커넥션 풀을 생성합니다.
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}



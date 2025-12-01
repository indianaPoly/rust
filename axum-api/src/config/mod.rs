use std::env;

/// 환경 변수에서 DATABASE_URL을 읽어옵니다.
///
/// 예시:
/// - `postgres://user:password@localhost:5432/mydb`
pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("환경 변수 DATABASE_URL 이 설정되어 있어야 합니다.")
}



## axum REST API 템플릿 (MVC 스타일)

간단한 axum 기반 REST API 서버 템플릿으로, Spring MVC 느낌으로 역할별로 폴더를 분리해 둔 예시입니다.

### 실행 방법

1. **환경 변수 설정**

PostgreSQL을 기준으로 예시를 작성했습니다.

```bash
export DATABASE_URL="postgres://user:password@localhost:5432/mydb"
```

예시용 테이블:

```sql
CREATE TABLE items (
    id   SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);
```

2. **빌드 & 실행**

```bash
cargo run
```

서버는 기본적으로 `127.0.0.1:3000` 에서 실행됩니다.

3. **테스트용 엔드포인트**

- `GET /health`  
  - 헬스 체크
- `GET /api/v1/items`  
  - 아이템 목록 조회 (DB 사용)
- `GET /api/v1/items/:id`  
  - 단일 아이템 조회
- `POST /api/v1/items`  
  - 아이템 생성  
  - 요청 예시:

```bash
curl -X POST http://127.0.0.1:3000/api/v1/items \
  -H "Content-Type: application/json" \
  -d '{"name": "New Item"}'
```

### 폴더 구조 (역할 분리)

- `src/main.rs`  
  - 앱 엔트리 포인트, 트레이싱 초기화, DB 커넥션 풀 생성, 라우터 조립
- `src/config/`  
  - 환경 설정 (예: `DATABASE_URL` 읽기)
- `src/routes/`  
  - 전체 라우팅 정의 (URL ↔ 컨트롤러 매핑)
- `src/controllers/`  
  - 컨트롤러 계층 (요청/응답 처리, 서비스/리포지토리 호출)  
  - Spring의 `@RestController` 역할
- `src/models/`  
  - 도메인 모델, DTO 정의 (`Item`, `CreateItemRequest`, `HealthResponse` 등)
- `src/db/`  
  - DB 관련 코드  
  - `pool.rs` : DB 커넥션 풀 생성  
  - `item_repository.rs` : `items` 테이블에 대한 쿼리 담당 (Repository 계층)
- `src/error/`  
  - 공통 에러 타입(`AppError`) 및 `IntoResponse` 구현 → JSON 에러 응답 통합 처리

이 구조를 기반으로 `service` 계층을 추가하거나, 도메인별 서브 폴더(`user`, `order` 등)를 만들어 확장하면 됩니다.




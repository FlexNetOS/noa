use axum::{
    body::{to_bytes, Body},
    http::{header, Request, StatusCode},
};
use noa_api::{db::Database, state::AppState, Server};
use rusqlite::OptionalExtension;
use serde::Deserialize;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Debug, Deserialize)]
struct WhoamiUser {
    id: String,
    email: Option<String>,
    name: Option<String>,
    role: String,
}

#[derive(Debug, Deserialize)]
struct WhoamiResponse {
    authenticated: bool,
    user: Option<WhoamiUser>,
}

#[tokio::test]
async fn auth_tables_exist_after_migrations() {
    let db_path = std::env::temp_dir().join(format!("noa-api-auth-parity-{}.db", uuid::Uuid::new_v4()));
    let _ = std::fs::remove_file(&db_path);

    let db = Database::new(&db_path).await.expect("db init");

    db.with_conn(|conn| {
        for table in [
            "auth_users",
            "auth_accounts",
            "auth_sessions",
            "auth_verification_tokens",
            "auth_oauth_states",
        ] {
            let exists: Option<String> = conn
                .query_row(
                    "SELECT name FROM sqlite_master WHERE type='table' AND name = ?1",
                    [table],
                    |row| row.get(0),
                )
                .optional()?;

            assert!(exists.is_some(), "expected table {table} to exist");
        }

        Ok(())
    })
    .expect("schema verification");

    let _ = std::fs::remove_file(&db_path);
}

#[tokio::test]
async fn whoami_requires_valid_session_cookie() {
    let db_path = std::env::temp_dir().join(format!("noa-api-whoami-{}.db", uuid::Uuid::new_v4()));
    let _ = std::fs::remove_file(&db_path);

    let db = Arc::new(Database::new(&db_path).await.expect("db init"));
    let state = AppState::from_env(db.clone());
    let app = Server::router(state);

    // No cookie -> authenticated=false
    let res = app
        .clone()
        .oneshot(Request::builder().uri("/api/v1/auth/whoami").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let parsed: WhoamiResponse = serde_json::from_slice(&body).unwrap();
    assert!(!parsed.authenticated);
    assert!(parsed.user.is_none());

    // Insert user + session
    let user_id = uuid::Uuid::new_v4().to_string();
    let session_id = uuid::Uuid::new_v4().to_string();

    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;
        tx.execute(
            "INSERT INTO auth_users (id, email, name, role) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![user_id, "test@example.com", "Test User", "admin"],
        )?;
        tx.execute(
            "INSERT INTO auth_sessions (id, user_id, expires_at) VALUES (?1, ?2, datetime('now', '+1 day'))",
            rusqlite::params![session_id, user_id],
        )?;
        tx.commit()?;
        Ok(())
    })
    .unwrap();

    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/whoami")
                .header(header::COOKIE, format!("noa_session={}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let parsed: WhoamiResponse = serde_json::from_slice(&body).unwrap();

    assert!(parsed.authenticated);
    assert_eq!(parsed.user.as_ref().unwrap().id, user_id);
    assert_eq!(parsed.user.as_ref().unwrap().email.as_deref(), Some("test@example.com"));
    assert_eq!(parsed.user.as_ref().unwrap().role, "admin");

    let _ = std::fs::remove_file(&db_path);
}

#[tokio::test]
async fn credentials_login_sets_cookie_and_logout_revokes_session() {
    let db_path = std::env::temp_dir().join(format!("noa-api-login-{}.db", uuid::Uuid::new_v4()));
    let _ = std::fs::remove_file(&db_path);

    let db = Arc::new(Database::new(&db_path).await.expect("db init"));
    let state = AppState::from_env(db.clone());
    let app = Server::router(state);

    let user_id = uuid::Uuid::new_v4().to_string();
    let password_hash = bcrypt::hash("password123", 12).expect("bcrypt hash");

    db.with_conn(|conn| {
        conn.execute(
            "INSERT INTO auth_users (id, email, name, role, password_hash) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![user_id, "test@example.com", "Test User", "user", password_hash],
        )?;
        Ok(())
    })
    .unwrap();

    // Wrong password -> 401
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::json!({"email":"test@example.com","password":"nope"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // Correct password -> 200 + Set-Cookie
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::json!({"email":"test@example.com","password":"password123"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let set_cookie = res
        .headers()
        .get(header::SET_COOKIE)
        .expect("set-cookie header")
        .to_str()
        .unwrap()
        .to_string();

    let session_id = set_cookie
        .split(';')
        .next()
        .unwrap()
        .strip_prefix("noa_session=")
        .unwrap()
        .to_string();

    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let parsed: WhoamiResponse = serde_json::from_slice(&body).unwrap();
    assert!(parsed.authenticated);

    // whoami with cookie -> authenticated
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/whoami")
                .header(header::COOKIE, format!("noa_session={}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let parsed: WhoamiResponse = serde_json::from_slice(&body).unwrap();
    assert!(parsed.authenticated);

    // logout -> clears cookie + revokes
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/logout")
                .header(header::COOKIE, format!("noa_session={}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    // old cookie should no longer authenticate (revoked_at)
    let res = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/auth/whoami")
                .header(header::COOKIE, format!("noa_session={}", session_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let parsed: WhoamiResponse = serde_json::from_slice(&body).unwrap();
    assert!(!parsed.authenticated);

    let _ = std::fs::remove_file(&db_path);
}

#[tokio::test]
async fn credentials_signup_creates_user_and_allows_login() {
    let db_path = std::env::temp_dir().join(format!("noa-api-signup-{}.db", uuid::Uuid::new_v4()));
    let _ = std::fs::remove_file(&db_path);

    let db = Arc::new(Database::new(&db_path).await.expect("db init"));
    let state = AppState::from_env(db.clone());
    let app = Server::router(state);

    // Signup -> 201
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/signup")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::json!({"email":"new@example.com","password":"password123","name":"New User"})
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::CREATED);
    let body = to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let created_user: WhoamiUser = serde_json::from_slice(&body).unwrap();
    assert_eq!(created_user.email.as_deref(), Some("new@example.com"));
    assert_eq!(created_user.role, "user");

    // Duplicate signup -> 400
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/signup")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::json!({"email":"new@example.com","password":"password123"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // Login should now succeed
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    serde_json::json!({"email":"new@example.com","password":"password123"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);

    let _ = std::fs::remove_file(&db_path);
}

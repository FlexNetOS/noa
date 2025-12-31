//! Authentication endpoints
//!
//! Minimal Rust-native auth surface for parity with NextAuth:
//! - whoami (session introspection)
//! - credentials login/logout
//! - GitHub OAuth start + callback
//! - Google OAuth start + callback

use axum::{
    extract::{Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use rusqlite::OptionalExtension;

use crate::{
    db::Database,
    state::{ApiErrorBody, AppState},
};

fn cookie_secure() -> bool {
    std::env::var("NOA_COOKIE_SECURE")
        .ok()
        .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

fn build_session_cookie_value(session_id: &str) -> String {
    let mut cookie = format!(
        "noa_session={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=2592000",
        session_id
    );
    if cookie_secure() {
        cookie.push_str("; Secure");
    }
    cookie
}

fn build_clear_session_cookie_value() -> String {
    let mut cookie = "noa_session=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0".to_string();
    if cookie_secure() {
        cookie.push_str("; Secure");
    }
    cookie
}

fn sanitize_redirect_to(value: Option<String>) -> Option<String> {
    let value = value?;
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    // Only allow relative-path redirects to avoid open redirects.
    // - must start with '/'
    // - must not start with '//' (scheme-relative)
    // - must not contain backslashes
    if !value.starts_with('/') || value.starts_with("//") || value.contains('\\') {
        return None;
    }

    Some(value.to_string())
}

#[derive(Debug, Serialize)]
pub struct WhoamiUser {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct WhoamiResponse {
    pub authenticated: bool,
    pub user: Option<WhoamiUser>,
}

pub async fn whoami(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let Some(session_id) = get_cookie_value(&headers, "noa_session") else {
        return (StatusCode::OK, Json(WhoamiResponse { authenticated: false, user: None }))
            .into_response();
    };

    let db = state.db.clone();
    let result = tokio::task::spawn_blocking(move || lookup_session_user(&db, &session_id)).await;

    match result {
        Ok(Ok(Some(user))) => (StatusCode::OK, Json(WhoamiResponse { authenticated: true, user: Some(user) }))
            .into_response(),
        Ok(Ok(None)) => (StatusCode::OK, Json(WhoamiResponse { authenticated: false, user: None }))
            .into_response(),
        Ok(Err(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorBody {
                error: format!("DB error: {e:#}"),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorBody {
                error: format!("Join error: {e}"),
            }),
        )
            .into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct CredentialsLoginRequest {
    pub email: String,
    pub password: String,
    pub redirect_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CredentialsSignupRequest {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<CredentialsSignupRequest>,
) -> impl IntoResponse {
    let email = payload.email.trim().to_string();
    if email.is_empty() || payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: "Email and password are required".to_string(),
            }),
        )
            .into_response();
    }

    let name = payload
        .name
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .or_else(|| email.split('@').next().map(|v| v.to_string()));

    let db = state.db.clone();
    let password = payload.password;

    let result = tokio::task::spawn_blocking(move || create_user_with_password(&db, &email, &password, name.as_deref()))
        .await;

    match result {
        Ok(Ok(Some(user))) => (StatusCode::CREATED, Json(user)).into_response(),
        Ok(Ok(None)) => (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: "User already exists".to_string(),
            }),
        )
            .into_response(),
        Ok(Err(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorBody {
                error: format!("DB error: {e:#}"),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiErrorBody {
                error: format!("Join error: {e}"),
            }),
        )
            .into_response(),
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<CredentialsLoginRequest>,
) -> impl IntoResponse {
    if payload.email.trim().is_empty() || payload.password.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: "Email and password required".to_string(),
            }),
        )
            .into_response();
    }

    let db = state.db.clone();
    let email = payload.email.trim().to_string();
    let password = payload.password;
    let redirect_to = payload.redirect_to;

    let result = tokio::task::spawn_blocking(move || login_with_password(&db, &email, &password)).await;

    let (session_id, user) = match result {
        Ok(Ok(Some(v))) => v,
        Ok(Ok(None)) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorBody {
                    error: "Invalid credentials".to_string(),
                }),
            )
                .into_response();
        }
        Ok(Err(e)) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("DB error: {e:#}"),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Join error: {e}"),
                }),
            )
                .into_response();
        }
    };

    let cookie = build_session_cookie_value(&session_id);

    if redirect_to.is_some() {
        let redirect = sanitize_redirect_to(redirect_to).unwrap_or_else(|| "/".to_string());
        let mut headers = HeaderMap::new();
        headers.insert(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie).unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        headers.insert(
            header::LOCATION,
            HeaderValue::from_str(&redirect).unwrap_or_else(|_| HeaderValue::from_static("/")),
        );
        return (StatusCode::FOUND, headers).into_response();
    }

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).unwrap_or_else(|_| HeaderValue::from_static("")),
    );

    (
        StatusCode::OK,
        headers,
        Json(WhoamiResponse {
            authenticated: true,
            user: Some(user),
        }),
    )
        .into_response()
}

pub async fn logout(State(state): State<AppState>, headers: HeaderMap) -> impl IntoResponse {
    let session_id = get_cookie_value(&headers, "noa_session");
    if let Some(session_id) = session_id {
        let db = state.db.clone();
        let _ = tokio::task::spawn_blocking(move || revoke_session(&db, &session_id)).await;
    }

    let cookie = build_clear_session_cookie_value();
    let mut out_headers = HeaderMap::new();
    out_headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).unwrap_or_else(|_| HeaderValue::from_static("")),
    );
    (StatusCode::OK, out_headers).into_response()
}

#[derive(Debug, Deserialize)]
pub struct GithubStartQuery {
    pub redirect_to: Option<String>,
}

pub async fn github_start(
    State(state): State<AppState>,
    Query(q): Query<GithubStartQuery>,
) -> impl IntoResponse {
    let client_id = match std::env::var("GITHUB_CLIENT_ID") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: "Missing GITHUB_CLIENT_ID".to_string(),
                }),
            )
                .into_response();
        }
    };

    let callback_url = std::env::var("NOA_AUTH_GITHUB_CALLBACK_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "http://127.0.0.1:3001/api/v1/auth/github/callback".to_string());

    let scope = "read:user user:email".to_string();
    let state_value = uuid::Uuid::new_v4().to_string();

    let db = state.db.clone();
    let redirect_to = sanitize_redirect_to(q.redirect_to);
    let state_to_store = state_value.clone();
    let scope_to_store = scope.clone();

    let inserted = tokio::task::spawn_blocking(move || {
        insert_oauth_state(&db, &state_to_store, "github", redirect_to.as_deref(), &scope_to_store)
    })
    .await;

    match inserted {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("DB error: {e:#}"),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Join error: {e}"),
                }),
            )
                .into_response();
        }
    }

    let mut url = match reqwest::Url::parse("https://github.com/login/oauth/authorize") {
        Ok(u) => u,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Failed to build GitHub authorize URL: {e}"),
                }),
            )
                .into_response();
        }
    };

    url.query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("redirect_uri", &callback_url)
        .append_pair("scope", &scope)
        .append_pair("state", &state_value);

    axum::response::Redirect::temporary(url.as_str()).into_response()
}

#[derive(Debug, Deserialize)]
pub struct GoogleStartQuery {
    pub redirect_to: Option<String>,
}

pub async fn google_start(
    State(state): State<AppState>,
    Query(q): Query<GoogleStartQuery>,
) -> impl IntoResponse {
    let client_id = match std::env::var("GOOGLE_CLIENT_ID") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: "Missing GOOGLE_CLIENT_ID".to_string(),
                }),
            )
                .into_response();
        }
    };

    let callback_url = std::env::var("NOA_AUTH_GOOGLE_CALLBACK_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "http://127.0.0.1:3001/api/v1/auth/google/callback".to_string());

    // Mirror NextAuth params: prompt=consent, access_type=offline, response_type=code
    let scope = "openid email profile".to_string();
    let state_value = uuid::Uuid::new_v4().to_string();

    let db = state.db.clone();
    let redirect_to = sanitize_redirect_to(q.redirect_to);
    let state_to_store = state_value.clone();
    let scope_to_store = scope.clone();

    let inserted = tokio::task::spawn_blocking(move || {
        insert_oauth_state(&db, &state_to_store, "google", redirect_to.as_deref(), &scope_to_store)
    })
    .await;

    match inserted {
        Ok(Ok(())) => {}
        Ok(Err(e)) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("DB error: {e:#}"),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Join error: {e}"),
                }),
            )
                .into_response();
        }
    }

    let mut url = match reqwest::Url::parse("https://accounts.google.com/o/oauth2/v2/auth") {
        Ok(u) => u,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Failed to build Google authorize URL: {e}"),
                }),
            )
                .into_response();
        }
    };

    url.query_pairs_mut()
        .append_pair("client_id", &client_id)
        .append_pair("redirect_uri", &callback_url)
        .append_pair("response_type", "code")
        .append_pair("scope", &scope)
        .append_pair("access_type", "offline")
        .append_pair("prompt", "consent")
        .append_pair("state", &state_value);

    axum::response::Redirect::temporary(url.as_str()).into_response()
}

#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(q): Query<GoogleCallbackQuery>,
) -> impl IntoResponse {
    let Some(code) = q.code else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: "Missing 'code'".to_string(),
            }),
        )
            .into_response();
    };

    let Some(state_value) = q.state else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: "Missing 'state'".to_string(),
            }),
        )
            .into_response();
    };

    let client_id = match std::env::var("GOOGLE_CLIENT_ID") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: "Missing GOOGLE_CLIENT_ID".to_string(),
                }),
            )
                .into_response();
        }
    };

    let client_secret = match std::env::var("GOOGLE_CLIENT_SECRET") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: "Missing GOOGLE_CLIENT_SECRET".to_string(),
                }),
            )
                .into_response();
        }
    };

    let callback_url = std::env::var("NOA_AUTH_GOOGLE_CALLBACK_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "http://127.0.0.1:3001/api/v1/auth/google/callback".to_string());

    // Consume and validate state (CSRF protection)
    let db = state.db.clone();
    let state_for_db = state_value.clone();
    let consume_result =
        tokio::task::spawn_blocking(move || consume_oauth_state(&db, &state_for_db, "google")).await;

    let redirect_to = match consume_result {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorBody {
                    error: format!("Invalid or expired state: {e:#}"),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Join error: {e}"),
                }),
            )
                .into_response();
        }
    };

    // Exchange code -> access token
    let http = reqwest::Client::new();
    let token_resp = match http
        .post("https://oauth2.googleapis.com/token")
        .form(&[
            ("code", code.as_str()),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("redirect_uri", callback_url.as_str()),
            ("grant_type", "authorization_code"),
        ])
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("Google token exchange failed: {e}"),
                }),
            )
                .into_response();
        }
    };

    if !token_resp.status().is_success() {
        let status = token_resp.status();
        let body = token_resp.text().await.unwrap_or_default();
        return (
            StatusCode::BAD_GATEWAY,
            Json(ApiErrorBody {
                error: format!("Google token exchange failed with {status}: {body}"),
            }),
        )
            .into_response();
    }

    let token_body: GoogleTokenResponse = match token_resp.json().await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("Failed to parse Google token response: {e}"),
                }),
            )
                .into_response();
        }
    };

    let Some(access_token) = token_body.access_token.clone() else {
        return (
            StatusCode::BAD_GATEWAY,
            Json(ApiErrorBody {
                error: "Google token exchange returned no access_token".to_string(),
            }),
        )
            .into_response();
    };

    // Fetch user identity (userinfo)
    let userinfo_resp = match http
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(&access_token)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("Google userinfo fetch failed: {e}"),
                }),
            )
                .into_response();
        }
    };

    if !userinfo_resp.status().is_success() {
        let status = userinfo_resp.status();
        let body = userinfo_resp.text().await.unwrap_or_default();
        return (
            StatusCode::BAD_GATEWAY,
            Json(ApiErrorBody {
                error: format!("Google userinfo fetch failed with {status}: {body}"),
            }),
        )
            .into_response();
    }

    let userinfo: GoogleUserInfo = match userinfo_resp.json().await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("Failed to parse Google userinfo response: {e}"),
                }),
            )
                .into_response();
        }
    };

    // Create/update user+account+session
    let db = state.db.clone();
    let upsert = tokio::task::spawn_blocking(move || upsert_google_user_and_session(&db, &userinfo, &token_body))
        .await;

    let (session_id, _user_id) = match upsert {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("DB error: {e:#}"),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Join error: {e}"),
                }),
            )
                .into_response();
        }
    };

    let cookie = build_session_cookie_value(&session_id);

    let final_redirect = sanitize_redirect_to(redirect_to)
        .or_else(|| sanitize_redirect_to(std::env::var("NOA_AUTH_SUCCESS_REDIRECT").ok()))
        .unwrap_or_else(|| "/".to_string());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).unwrap_or_else(|_| HeaderValue::from_static("")),
    );
    headers.insert(
        header::LOCATION,
        HeaderValue::from_str(&final_redirect)
            .unwrap_or_else(|_| HeaderValue::from_static("/")),
    );

    (StatusCode::FOUND, headers).into_response()
}

#[derive(Debug, Deserialize)]
pub struct GithubCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
}

pub async fn github_callback(
    State(state): State<AppState>,
    Query(q): Query<GithubCallbackQuery>,
) -> impl IntoResponse {
    let Some(code) = q.code else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: "Missing 'code'".to_string(),
            }),
        )
            .into_response();
    };

    let Some(state_value) = q.state else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiErrorBody {
                error: "Missing 'state'".to_string(),
            }),
        )
            .into_response();
    };

    let client_id = match std::env::var("GITHUB_CLIENT_ID") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: "Missing GITHUB_CLIENT_ID".to_string(),
                }),
            )
                .into_response();
        }
    };

    let client_secret = match std::env::var("GITHUB_CLIENT_SECRET") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: "Missing GITHUB_CLIENT_SECRET".to_string(),
                }),
            )
                .into_response();
        }
    };

    let callback_url = std::env::var("NOA_AUTH_GITHUB_CALLBACK_URL")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "http://127.0.0.1:3001/api/v1/auth/github/callback".to_string());

    // Consume and validate state (CSRF protection)
    let db = state.db.clone();
    let state_for_db = state_value.clone();
    let consume_result = tokio::task::spawn_blocking(move || consume_oauth_state(&db, &state_for_db, "github"))
        .await;

    let redirect_to = match consume_result {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorBody {
                    error: format!("Invalid or expired state: {e:#}"),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Join error: {e}"),
                }),
            )
                .into_response();
        }
    };

    // Exchange code -> access token
    let http = reqwest::Client::new();
    let token_resp = match http
        .post("https://github.com/login/oauth/access_token")
        .header(header::ACCEPT, "application/json")
        .header(header::USER_AGENT, "noa")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("code", code.as_str()),
            ("redirect_uri", callback_url.as_str()),
            ("state", state_value.as_str()),
        ])
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("GitHub token exchange failed: {e}"),
                }),
            )
                .into_response();
        }
    };

    let token_body: GithubTokenResponse = match token_resp.json().await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("Failed to parse GitHub token response: {e}"),
                }),
            )
                .into_response();
        }
    };

    let Some(access_token) = token_body.access_token.clone() else {
        let msg = token_body
            .error_description
            .or(token_body.error)
            .unwrap_or_else(|| "Unknown error".to_string());
        return (
            StatusCode::BAD_GATEWAY,
            Json(ApiErrorBody {
                error: format!("GitHub token exchange returned no access_token: {msg}"),
            }),
        )
            .into_response();
    };

    // Fetch user identity
    let user_resp = match http
        .get("https://api.github.com/user")
        .header(header::USER_AGENT, "noa")
        .bearer_auth(&access_token)
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("GitHub user fetch failed: {e}"),
                }),
            )
                .into_response();
        }
    };

    let gh_user: GithubUser = match user_resp.json().await {
        Ok(v) => v,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(ApiErrorBody {
                    error: format!("Failed to parse GitHub user response: {e}"),
                }),
            )
                .into_response();
        }
    };

    // Fetch best email (optional)
    let emails_resp = http
        .get("https://api.github.com/user/emails")
        .header(header::USER_AGENT, "noa")
        .bearer_auth(&access_token)
        .send()
        .await;

    let email = match emails_resp {
        Ok(r) if r.status().is_success() => match r.json::<Vec<GithubEmail>>().await {
            Ok(emails) => pick_github_email(emails),
            Err(_) => None,
        },
        _ => None,
    };

    // Create/update user+account+session
    let db = state.db.clone();
    let upsert = tokio::task::spawn_blocking(move || {
        upsert_github_user_and_session(
            &db,
            &gh_user,
            email.as_deref(),
            &access_token,
            token_body.token_type.as_deref(),
            token_body.scope.as_deref(),
        )
    })
    .await;

    let (session_id, _user_id) = match upsert {
        Ok(Ok(v)) => v,
        Ok(Err(e)) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("DB error: {e:#}"),
                }),
            )
                .into_response();
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorBody {
                    error: format!("Join error: {e}"),
                }),
            )
                .into_response();
        }
    };

    let cookie = build_session_cookie_value(&session_id);

    let final_redirect = sanitize_redirect_to(redirect_to)
        .or_else(|| sanitize_redirect_to(std::env::var("NOA_AUTH_SUCCESS_REDIRECT").ok()))
        .unwrap_or_else(|| "/".to_string());

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        HeaderValue::from_str(&cookie).unwrap_or_else(|_| HeaderValue::from_static("")),
    );
    headers.insert(
        header::LOCATION,
        HeaderValue::from_str(&final_redirect)
            .unwrap_or_else(|_| HeaderValue::from_static("/")),
    );

    (StatusCode::FOUND, headers).into_response()
}

fn get_cookie_value(headers: &HeaderMap, name: &str) -> Option<String> {
    let raw = headers.get(header::COOKIE)?.to_str().ok()?;
    for part in raw.split(';') {
        let part = part.trim();
        if let Some(v) = part.strip_prefix(&format!("{name}=")) {
            return Some(v.to_string());
        }
    }
    None
}

fn lookup_session_user(db: &Arc<Database>, session_id: &str) -> anyhow::Result<Option<WhoamiUser>> {
    db.with_conn(|conn| {
        let mut stmt = conn.prepare(
            r#"
            SELECT u.id, u.email, u.name, u.role
            FROM auth_sessions s
            JOIN auth_users u ON u.id = s.user_id
            WHERE s.id = ?1
              AND s.revoked_at IS NULL
              AND s.expires_at > datetime('now')
            LIMIT 1
            "#,
        )?;

        let mut rows = stmt.query([session_id])?;
        let Some(row) = rows.next()? else {
            return Ok(None);
        };

        Ok(Some(WhoamiUser {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            role: row.get(3)?,
        }))
    })
}

fn revoke_session(db: &Arc<Database>, session_id: &str) -> anyhow::Result<()> {
    db.with_conn(|conn| {
        conn.execute(
            "UPDATE auth_sessions SET revoked_at = datetime('now') WHERE id = ?1 AND revoked_at IS NULL",
            rusqlite::params![session_id],
        )?;
        Ok(())
    })
}

fn login_with_password(
    db: &Arc<Database>,
    email: &str,
    password: &str,
) -> anyhow::Result<Option<(String, WhoamiUser)>> {
    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;

        let row: Option<(String, Option<String>, Option<String>, String, Option<String>)> = tx
            .query_row(
                r#"
                SELECT id, email, name, role, password_hash
                FROM auth_users
                WHERE email = ?1
                  AND disabled_at IS NULL
                LIMIT 1
                "#,
                rusqlite::params![email],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
            )
            .optional()?;

        let Some((user_id, user_email, user_name, user_role, password_hash)) = row else {
            return Ok(None);
        };

        let Some(password_hash) = password_hash else {
            return Ok(None);
        };

        let ok = bcrypt::verify(password, &password_hash)?;
        if !ok {
            return Ok(None);
        }

        let session_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO auth_sessions (id, user_id, expires_at) VALUES (?1, ?2, datetime('now', '+30 days'))",
            rusqlite::params![session_id, user_id],
        )?;

        tx.commit()?;
        Ok(Some((
            session_id,
            WhoamiUser {
                id: user_id,
                email: user_email,
                name: user_name,
                role: user_role,
            },
        )))
    })
}

fn create_user_with_password(
    db: &Arc<Database>,
    email: &str,
    password: &str,
    name: Option<&str>,
) -> anyhow::Result<Option<WhoamiUser>> {
    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;

        let existing: Option<String> = tx
            .query_row(
                "SELECT id FROM auth_users WHERE email = ?1 LIMIT 1",
                rusqlite::params![email],
                |r| r.get(0),
            )
            .optional()?;

        if existing.is_some() {
            return Ok(None);
        }

        let user_id = uuid::Uuid::new_v4().to_string();
        let password_hash = bcrypt::hash(password, 12)?;

        tx.execute(
            "INSERT INTO auth_users (id, email, name, password_hash, role) VALUES (?1, ?2, ?3, ?4, 'user')",
            rusqlite::params![user_id, email, name, password_hash],
        )?;

        tx.commit()?;

        Ok(Some(WhoamiUser {
            id: user_id,
            email: Some(email.to_string()),
            name: name.map(|v| v.to_string()),
            role: "user".to_string(),
        }))
    })
}

fn insert_oauth_state(
    db: &Arc<Database>,
    state: &str,
    provider: &str,
    redirect_to: Option<&str>,
    scope: &str,
) -> anyhow::Result<()> {
    db.with_conn(|conn| {
        conn.execute(
            r#"
            INSERT INTO auth_oauth_states (state, provider, expires_at, used_at, redirect_to, scope)
            VALUES (?1, ?2, datetime('now', '+10 minutes'), NULL, ?3, ?4)
            "#,
            rusqlite::params![state, provider, redirect_to, scope],
        )?;
        Ok(())
    })
}

fn consume_oauth_state(db: &Arc<Database>, state: &str, provider: &str) -> anyhow::Result<Option<String>> {
    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;

        let redirect_to: Option<String> = tx
            .query_row(
                "SELECT redirect_to FROM auth_oauth_states WHERE state = ?1 AND provider = ?2",
                rusqlite::params![state, provider],
                |row| row.get(0),
            )
            .optional()?;

        let changed = tx.execute(
            r#"
            UPDATE auth_oauth_states
            SET used_at = datetime('now')
            WHERE state = ?1
              AND provider = ?2
              AND used_at IS NULL
              AND expires_at > datetime('now')
            "#,
            rusqlite::params![state, provider],
        )?;

        if changed != 1 {
            anyhow::bail!("state not found, expired, or already used");
        }

        tx.commit()?;
        Ok(redirect_to)
    })
}

#[derive(Debug, Deserialize)]
struct GithubTokenResponse {
    access_token: Option<String>,
    token_type: Option<String>,
    scope: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubUser {
    id: u64,
    #[allow(dead_code)]
    login: String,
    name: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

fn pick_github_email(mut emails: Vec<GithubEmail>) -> Option<String> {
    // Prefer primary+verified, then primary, then verified, then first.
    emails.sort_by_key(|e| (!e.primary, !e.verified));
    emails.into_iter().map(|e| e.email).next()
}

fn upsert_github_user_and_session(
    db: &Arc<Database>,
    gh_user: &GithubUser,
    email: Option<&str>,
    access_token: &str,
    token_type: Option<&str>,
    scope: Option<&str>,
) -> anyhow::Result<(String, String)> {
    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;

        // If we already have a github account link, prefer that.
        let existing_user_id_by_account: Option<String> = tx
            .query_row(
                r#"
                SELECT user_id
                FROM auth_accounts
                WHERE provider = 'github' AND provider_account_id = ?1
                LIMIT 1
                "#,
                rusqlite::params![gh_user.id.to_string()],
                |row| row.get(0),
            )
            .optional()?;

        let user_id = if let Some(id) = existing_user_id_by_account {
            id
        } else if let Some(email) = email {
            // Try email match.
            let existing_user_id_by_email: Option<String> = tx
                .query_row(
                    "SELECT id FROM auth_users WHERE email = ?1 LIMIT 1",
                    rusqlite::params![email],
                    |row| row.get(0),
                )
                .optional()?;

            if let Some(id) = existing_user_id_by_email {
                id
            } else {
                let id = uuid::Uuid::new_v4().to_string();
                tx.execute(
                    r#"
                    INSERT INTO auth_users (id, email, email_verified_at, name, image, role)
                    VALUES (?1, ?2, datetime('now'), ?3, ?4, 'user')
                    "#,
                    rusqlite::params![id, email, gh_user.name.as_deref(), gh_user.avatar_url.as_deref()],
                )?;
                id
            }
        } else {
            // No email available; create a user without email.
            let id = uuid::Uuid::new_v4().to_string();
            tx.execute(
                r#"
                INSERT INTO auth_users (id, email, email_verified_at, name, image, role)
                VALUES (?1, NULL, NULL, ?2, ?3, 'user')
                "#,
                rusqlite::params![id, gh_user.name.as_deref(), gh_user.avatar_url.as_deref()],
            )?;
            id
        };

        // Upsert account link
        let account_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            r#"
            INSERT INTO auth_accounts (
                id, user_id, type, provider, provider_account_id,
                access_token, token_type, scope, refresh_token, expires_at, id_token, session_state
            )
            VALUES (?1, ?2, 'oauth', 'github', ?3, ?4, ?5, ?6, NULL, NULL, NULL, NULL)
            ON CONFLICT(provider, provider_account_id)
            DO UPDATE SET
                user_id = excluded.user_id,
                access_token = excluded.access_token,
                token_type = excluded.token_type,
                scope = excluded.scope,
                updated_at = datetime('now')
            "#,
            rusqlite::params![
                account_id,
                user_id,
                gh_user.id.to_string(),
                access_token,
                token_type,
                scope,
            ],
        )?;

        // Create session
        let session_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            r#"
            INSERT INTO auth_sessions (id, user_id, expires_at)
            VALUES (?1, ?2, datetime('now', '+30 days'))
            "#,
            rusqlite::params![session_id, user_id],
        )?;

        tx.commit()?;
        Ok((session_id, user_id))
    })
}

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<i64>,
    scope: Option<String>,
    token_type: Option<String>,
    id_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    sub: String,
    email: Option<String>,
    email_verified: Option<bool>,
    name: Option<String>,
    picture: Option<String>,
}

fn upsert_google_user_and_session(
    db: &Arc<Database>,
    userinfo: &GoogleUserInfo,
    token: &GoogleTokenResponse,
) -> anyhow::Result<(String, String)> {
    db.with_conn(|conn| {
        let tx = conn.unchecked_transaction()?;

        // If we already have a google account link, prefer that.
        let existing_user_id_by_account: Option<String> = tx
            .query_row(
                r#"
                SELECT user_id
                FROM auth_accounts
                WHERE provider = 'google' AND provider_account_id = ?1
                LIMIT 1
                "#,
                rusqlite::params![userinfo.sub],
                |row| row.get(0),
            )
            .optional()?;

        let user_id = if let Some(id) = existing_user_id_by_account {
            id
        } else if let Some(email) = userinfo.email.as_deref() {
            // Try email match.
            let existing_user_id_by_email: Option<String> = tx
                .query_row(
                    "SELECT id FROM auth_users WHERE email = ?1 LIMIT 1",
                    rusqlite::params![email],
                    |row| row.get(0),
                )
                .optional()?;

            if let Some(id) = existing_user_id_by_email {
                id
            } else {
                let id = uuid::Uuid::new_v4().to_string();
                if userinfo.email_verified.unwrap_or(false) {
                    tx.execute(
                        r#"
                        INSERT INTO auth_users (id, email, email_verified_at, name, image, role)
                        VALUES (?1, ?2, datetime('now'), ?3, ?4, 'user')
                        "#,
                        rusqlite::params![id, email, userinfo.name.as_deref(), userinfo.picture.as_deref()],
                    )?;
                } else {
                    tx.execute(
                        r#"
                        INSERT INTO auth_users (id, email, email_verified_at, name, image, role)
                        VALUES (?1, ?2, NULL, ?3, ?4, 'user')
                        "#,
                        rusqlite::params![id, email, userinfo.name.as_deref(), userinfo.picture.as_deref()],
                    )?;
                }
                id
            }
        } else {
            // No email available; create a user without email.
            let id = uuid::Uuid::new_v4().to_string();
            tx.execute(
                r#"
                INSERT INTO auth_users (id, email, email_verified_at, name, image, role)
                VALUES (?1, NULL, NULL, ?2, ?3, 'user')
                "#,
                rusqlite::params![id, userinfo.name.as_deref(), userinfo.picture.as_deref()],
            )?;
            id
        };

        // Upsert account link
        let account_id = uuid::Uuid::new_v4().to_string();

        let expires_at: Option<i64> = token.expires_in.map(|secs| {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            now + secs
        });

        tx.execute(
            r#"
            INSERT INTO auth_accounts (
                id, user_id, type, provider, provider_account_id,
                refresh_token, access_token, expires_at, token_type, scope, id_token, session_state
            )
            VALUES (?1, ?2, 'oauth', 'google', ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL)
            ON CONFLICT(provider, provider_account_id)
            DO UPDATE SET
                user_id = excluded.user_id,
                refresh_token = excluded.refresh_token,
                access_token = excluded.access_token,
                expires_at = excluded.expires_at,
                token_type = excluded.token_type,
                scope = excluded.scope,
                id_token = excluded.id_token,
                updated_at = datetime('now')
            "#,
            rusqlite::params![
                account_id,
                user_id,
                userinfo.sub,
                token.refresh_token.as_deref(),
                token.access_token.as_deref(),
                expires_at,
                token.token_type.as_deref(),
                token.scope.as_deref(),
                token.id_token.as_deref(),
            ],
        )?;

        // Create session
        let session_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            r#"
            INSERT INTO auth_sessions (id, user_id, expires_at)
            VALUES (?1, ?2, datetime('now', '+30 days'))
            "#,
            rusqlite::params![session_id, user_id],
        )?;

        tx.commit()?;
        Ok((session_id, user_id))
    })
}

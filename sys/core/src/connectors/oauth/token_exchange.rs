use serde::{Deserialize, Serialize};

use crate::error::{ApiError, NoaError, Result};

use super::client::OAuthClient;

/// OAuth token response payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default)]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub expires_in: Option<u64>,
    #[serde(default)]
    pub token_type: String,
    #[serde(default)]
    pub scope: Option<String>,
}

impl TokenResponse {
    pub fn is_expired(&self) -> bool {
        self.expires_in.unwrap_or_default() == 0
    }
}

/// Exchange an authorization code for access/refresh tokens
pub async fn exchange_code(client: &OAuthClient, code: &str) -> Result<TokenResponse> {
    let payload = [
        ("grant_type", "authorization_code".to_string()),
        ("code", code.to_string()),
        ("redirect_uri", client.config().redirect_uri.clone()),
        ("client_id", client.config().client_id.clone()),
        ("client_secret", client.config().client_secret.clone()),
    ];

    let response = client
        .client()
        .post(client.config().token_url.clone())
        .form(&payload)
        .send()
        .await
        .map_err(|e| NoaError::Internal {
            message: format!("Token exchange failed: {}", e),
            source: Some(Box::new(e)),
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(NoaError::Api(ApiError::BadRequest(format!(
            "Token endpoint returned {}: {}",
            status, body
        ))));
    }

    let tokens = response
        .json::<TokenResponse>()
        .await
        .map_err(|e| NoaError::Serialization(e.to_string()))?;

    Ok(tokens)
}

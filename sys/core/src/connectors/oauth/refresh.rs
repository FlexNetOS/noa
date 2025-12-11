use crate::error::{ApiError, NoaError, Result};

use super::client::OAuthClient;
use super::token_exchange::TokenResponse;

/// Refresh an existing access token using a refresh token
pub async fn refresh_token(
    client: &OAuthClient,
    refresh_token: &str,
) -> Result<TokenResponse> {
    let payload = [
        ("grant_type", "refresh_token".to_string()),
        ("refresh_token", refresh_token.to_string()),
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
            message: format!("Token refresh failed: {}", e),
            source: Some(Box::new(e)),
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(NoaError::Api(ApiError::Unauthorized(format!(
            "Refresh rejected {}: {}",
            status, body
        ))));
    }

    let tokens = response
        .json::<TokenResponse>()
        .await
        .map_err(|e| NoaError::Serialization(e.to_string()))?;

    Ok(tokens)
}

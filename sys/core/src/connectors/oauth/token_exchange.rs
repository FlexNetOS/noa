use crate::connectors::oauth::client::OAuthClient;
use crate::error::Result;

/// Exchange an authorization code for an access token.
pub async fn exchange_code(_client: &OAuthClient, _code: &str) -> Result<()> {
    // Placeholder implementation; real integrations will call the provider token endpoint.
    Ok(())
}


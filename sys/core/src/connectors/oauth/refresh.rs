use crate::connectors::oauth::client::OAuthClient;
use crate::error::Result;

/// Refresh an existing OAuth access token.
pub async fn refresh_token(_client: &OAuthClient, _refresh_token: &str) -> Result<()> {
    Ok(())
}


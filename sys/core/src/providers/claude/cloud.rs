use crate::error::Result;

pub async fn invoke_cloud(_payload: &str) -> Result<String> {
    Ok("claude-cloud-response".into())
}

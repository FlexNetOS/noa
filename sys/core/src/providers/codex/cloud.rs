use crate::error::Result;

pub async fn invoke_cloud(_input: &str) -> Result<String> {
    Ok("codex-cloud-response".into())
}

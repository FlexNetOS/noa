use crate::error::Result;

pub async fn invoke_cloud(_payload: &str) -> Result<String> {
    Ok("abacus-cloud-response".into())
}

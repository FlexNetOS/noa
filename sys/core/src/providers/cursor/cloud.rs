use crate::error::Result;

pub async fn call_cloud(_input: &str) -> Result<String> {
    Ok("cursor-cloud-response".into())
}

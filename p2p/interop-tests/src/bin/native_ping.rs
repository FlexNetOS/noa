use anyhow::Result;

mod configs;

#[tokio::main]
async fn main() -> Result<()> {
    let configs = configs::configs::from_env()?;

    let report = interop_tests::run_test(
        &configs.transport,
        &configs.ip,
        configs.is_dialer,
        configs.test_timeout,
        &configs.redis_addr,
        configs.sec_protocol,
        configs.muxer,
    )
    .await?;

    println!("{}", serde_json::to_string(&report)?);

    Ok(())
}

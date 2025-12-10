use noa_api::{db::Database, Server};
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "noa_api=debug,tower_http=debug,sqlx=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let current_dir = env::current_dir()?;
    let data_dir = current_dir.join("data");
    std::fs::create_dir_all(&data_dir)?;

    let db_path = data_dir.join("noa.db");

    tracing::info!("Initializing database at {:?}", db_path);
    let db = Database::new(&db_path).await?;
    tracing::info!("Database initialized successfully");

    db.health_check().await?;
    tracing::info!("Database health check passed");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let server = Server::new(addr);

    tracing::info!("Starting NOA API server on {}", addr);
    server.run().await?;

    Ok(())
}

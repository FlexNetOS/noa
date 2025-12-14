use crate::error::Result;
use crate::providers::{
    health::check_provider,
    registry::{disable_provider, enable_provider, provider_by_id, providers},
    ProviderStatus,
};

/// List all providers
pub async fn list() -> Result<()> {
    let providers = providers()?;
    println!("{:<12} {:<8} {:<6} {}", "id", "kind", "prio", "status");
    for p in providers {
        println!(
            "{:<12} {:<8} {:<6} {:?}",
            p.id, p.kind, p.priority, p.status
        );
    }
    Ok(())
}

/// Show provider status
pub async fn status(name: String) -> Result<()> {
    if let Some(p) = provider_by_id(&name)? {
        println!("{} => {:?}", p.id, p.status);
    } else {
        println!("provider not found: {}", name);
    }
    Ok(())
}

/// Enable provider
pub async fn enable(name: String) -> Result<()> {
    if enable_provider(&name)? {
        println!("enabled {}", name);
    } else {
        println!("provider not found: {}", name);
    }
    Ok(())
}

/// Disable provider
pub async fn disable(name: String) -> Result<()> {
    if disable_provider(&name)? {
        println!("disabled {}", name);
    } else {
        println!("provider not found: {}", name);
    }
    Ok(())
}

/// Test provider connectivity (stub)
pub async fn test(name: String) -> Result<()> {
    if let Some(p) = provider_by_id(&name)? {
        let health = check_provider(&p).await?;
        let label = match health {
            ProviderStatus::Ready => "ready",
            ProviderStatus::Busy => "busy",
            ProviderStatus::Degraded => "degraded",
            ProviderStatus::Offline => "offline",
            ProviderStatus::Unknown => "unknown",
        };
        println!("{} health: {}", p.id, label);
    } else {
        println!("provider not found: {}", name);
    }
    Ok(())
}

use crate::error::Result;
use crate::healing::{SelfHealingOrchestrator, HealingStatus};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum HealingCmd {
    Status,
    Events,
}

pub async fn execute(cmd: HealingCmd, orchestrator: Option<Arc<SelfHealingOrchestrator>>) -> Result<()> {
    match cmd {
        HealingCmd::Status => {
            if let Some(orch) = orchestrator {
                let events = orch.get_active_events().await;
                let active = events.iter().filter(|e| !matches!(e.status, HealingStatus::Resolved | HealingStatus::Failed)).count();
                let resolved = events.iter().filter(|e| e.status == HealingStatus::Resolved).count();
                let escalated = events.iter().filter(|e| e.escalated).count();
                
                println!("Self-Healing Status");
                println!("{:-<60}", "");
                println!("Active incidents: {}", active);
                println!("Resolved: {}", resolved);
                println!("Escalated: {}", escalated);
                println!("Total events: {}", events.len());
            } else {
                println!("Healing orchestrator not initialized");
            }
            Ok(())
        }
        HealingCmd::Events => {
            if let Some(orch) = orchestrator {
                let events = orch.get_active_events().await;
                println!("Healing Events");
                println!("{:-<80}", "");
                for event in events {
                    println!("ID: {}", event.id);
                    println!("Component: {} ({})", event.component_id, event.component_type);
                    println!("Status: {:?}", event.status);
                    println!("Detected: {}", event.detected_at.format("%Y-%m-%d %H:%M:%S"));
                    if let Some(ref cause) = event.root_cause {
                        println!("Root Cause: {}", cause);
                    }
                    if let Some(ref fix) = event.fix_applied {
                        println!("Fix Applied: {}", fix);
                    }
                    println!("Attempts: {}", event.fix_attempts);
                    println!("{:-<80}", "");
                }
            } else {
                println!("Healing orchestrator not initialized");
            }
            Ok(())
        }
    }
}

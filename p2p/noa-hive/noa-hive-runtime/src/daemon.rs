//! Daemon implementation for NOA-Hive.

use noa_hive_configs::configs;
use noa_hive_core::PeerId;
use noa_hive_stack::{HiveSwarm, Swarmconfigs};
use tokio::sync::mpsc;
use tracing::{info, warn, error};

use crate::{DaemonCommand, DaemonEvent, DaemonHandle, StateManager};

/// The NOA-Hive daemon.
pub struct Daemon {
    configs: configs,
    peer_id: PeerId,
    state_manager: StateManager,
}

impl Daemon {
    /// Create a new daemon with the given configsuration.
    pub fn new(configs: configs) -> anyhow::Result<Self> {
        // Load or generate identity
        let identity_path = configs.storage.data_dir.join(&configs.storage.identity_file);
        let (keypair, peer_id) = noa_hive_stack::load_or_generate_identity(&identity_path)?;

        // Initialize state manager
        let state_manager = StateManager::new(&configs)?;

        Ok(Self {
            configs,
            peer_id,
            state_manager,
        })
    }

    /// Get the local peer ID.
    pub fn peer_id(&self) -> &PeerId {
        &self.peer_id
    }

    /// Run the daemon, returning a handle for communication.
    pub async fn run(self) -> anyhow::Result<DaemonHandle> {
        let (command_tx, mut command_rx) = mpsc::channel::<DaemonCommand>(256);
        let (event_tx, event_rx) = mpsc::channel::<DaemonEvent>(256);

        let peer_id = self.peer_id.clone();
        let configs = self.configs.clone();

        // Spawn the main daemon loop
        tokio::spawn(async move {
            info!(peer_id = %peer_id, "NOA-Hive daemon starting");

            // Emit started event
            let _ = event_tx.send(DaemonEvent::Started { peer_id: peer_id.clone() }).await;

            // Main command loop
            loop {
                tokio::select! {
                    Some(command) = command_rx.recv() => {
                        match command {
                            DaemonCommand::Shutdown => {
                                info!("Received shutdown command");
                                let _ = event_tx.send(DaemonEvent::Shutdown).await;
                                break;
                            }
                            DaemonCommand::WhoAmI(reply) => {
                                let _ = reply.send(peer_id.clone());
                            }
                            DaemonCommand::Subscribe(topic) => {
                                info!(topic = %topic, "Subscribing to topic");
                                // TODO: Implement subscription
                            }
                            DaemonCommand::Unsubscribe(topic) => {
                                info!(topic = %topic, "Unsubscribing from topic");
                                // TODO: Implement unsubscription
                            }
                            DaemonCommand::Publish { topic, data } => {
                                info!(topic = %topic, size = data.len(), "Publishing message");
                                // TODO: Implement publishing
                            }
                            DaemonCommand::DhtPut { key, value } => {
                                info!(key = %key, size = value.len(), "Storing in DHT");
                                // TODO: Implement DHT put
                            }
                            DaemonCommand::DhtGet { key, reply } => {
                                info!(key = %key, "Getting from DHT");
                                // TODO: Implement DHT get
                                let _ = reply.send(None);
                            }
                        }
                    }
                    // TODO: Handle swarm events, loro updates, iroh events
                }
            }

            info!("NOA-Hive daemon stopped");
        });

        Ok(DaemonHandle::new(command_tx, event_rx))
    }
}

/// Builder for configsuring the daemon.
pub struct DaemonBuilder {
    configs: configs,
}

impl DaemonBuilder {
    /// Create a new daemon builder with default configsuration.
    pub fn new() -> Self {
        Self {
            configs: configs::default(),
        }
    }

    /// Use a specific configsuration.
    pub fn with_configs(mut self, configs: configs) -> Self {
        self.configs = configs;
        self
    }

    /// Load configsuration from a file.
    pub fn with_configs_file(mut self, path: &std::path::Path) -> anyhow::Result<Self> {
        self.configs = configs::load(path)?;
        Ok(self)
    }

    /// Build the daemon.
    pub fn build(self) -> anyhow::Result<Daemon> {
        Daemon::new(self.configs)
    }
}

impl Default for DaemonBuilder {
    fn default() -> Self {
        Self::new()
    }
}

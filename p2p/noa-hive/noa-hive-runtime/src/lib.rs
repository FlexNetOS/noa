//! NOA-Hive Runtime
//!
//! The daemon runtime that orchestrates:
//! - P2P networking via noa-hive-stack
//! - State synchronization via loro
//! - Binary distribution via iroh
//! - gRPC API for client communication
//!
//! Derived from hyveos runtime with NOA-specific modifications.

pub mod daemon;
pub mod state;
pub mod grpc;

pub use daemon::Daemon;
pub use state::StateManager;

use noa_hive_configs::configs;
use noa_hive_core::PeerId;
use tokio::sync::mpsc;

/// Commands that can be sent to the daemon.
#[derive(Debug)]
pub enum DaemonCommand {
    /// Shutdown the daemon gracefully.
    Shutdown,
    /// Get the local peer ID.
    WhoAmI(tokio::sync::oneshot::Sender<PeerId>),
    /// Subscribe to a topic.
    Subscribe(String),
    /// Unsubscribe from a topic.
    Unsubscribe(String),
    /// Publish a message to a topic.
    Publish { topic: String, data: Vec<u8> },
    /// Store a value in the DHT.
    DhtPut { key: String, value: Vec<u8> },
    /// Get a value from the DHT.
    DhtGet { key: String, reply: tokio::sync::oneshot::Sender<Option<Vec<u8>>> },
}

/// Events emitted by the daemon.
#[derive(Debug, Clone)]
pub enum DaemonEvent {
    /// Daemon started successfully.
    Started { peer_id: PeerId },
    /// A peer connected.
    PeerConnected(PeerId),
    /// A peer disconnected.
    PeerDisconnected(PeerId),
    /// Received a message on a subscribed topic.
    Message { topic: String, data: Vec<u8>, from: Option<PeerId> },
    /// State updated via CRDT.
    StateUpdated { room_id: String },
    /// Daemon is shutting down.
    Shutdown,
}

/// Handle for communicating with the daemon.
pub struct DaemonHandle {
    command_tx: mpsc::Sender<DaemonCommand>,
    event_rx: mpsc::Receiver<DaemonEvent>,
}

impl DaemonHandle {
    /// Create a new daemon handle.
    pub fn new(
        command_tx: mpsc::Sender<DaemonCommand>,
        event_rx: mpsc::Receiver<DaemonEvent>,
    ) -> Self {
        Self { command_tx, event_rx }
    }

    /// Send a command to the daemon.
    pub async fn send(&self, command: DaemonCommand) -> anyhow::Result<()> {
        self.command_tx.send(command).await?;
        Ok(())
    }

    /// Receive the next event from the daemon.
    pub async fn recv(&mut self) -> Option<DaemonEvent> {
        self.event_rx.recv().await
    }

    /// Get the local peer ID.
    pub async fn whoami(&self) -> anyhow::Result<PeerId> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.send(DaemonCommand::WhoAmI(tx)).await?;
        Ok(rx.await?)
    }

    /// Shutdown the daemon.
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        self.send(DaemonCommand::Shutdown).await
    }
}

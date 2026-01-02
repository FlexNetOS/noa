//! NOA-Hive SDK
//!
//! Client library for interacting with the NOA-Hive daemon.
//! Provides a high-level API for P2P operations.
//!
//! # Example
//!
//! ```rust,ignore
//! use noa_hive_sdk::Client;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::connect("http://127.0.0.1:50051").await?;
//!     
//!     // Get our peer ID
//!     let peer_id = client.whoami().await?;
//!     println!("Connected as: {}", peer_id);
//!     
//!     // Publish to a topic
//!     client.pubsub().publish("my-topic", b"hello").await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod pubsub;
pub mod dht;
pub mod state;

pub use client::Client;
pub use noa_hive_core::{PeerId, Error, Result};

/// Default gRPC endpoint for the daemon.
pub const DEFAULT_ENDPOINT: &str = "http://127.0.0.1:50051";

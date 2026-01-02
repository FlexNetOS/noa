//! NOA API Client
//!
//! Shared API client for connecting to the NOA backend.
//! Used by the Dioxus UI and other Rust clients.
//!
//! # Example
//!
//! ```rust,ignore
//! use noa_api_client::{Client, ChatRequest};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let client = Client::new("http://localhost:3001");
//!     
//!     let response = client.chat(ChatRequest {
//!         message: "Hello".to_string(),
//!         provider: None,
//!         history: None,
//!         stream: false,
//!     }).await?;
//!     
//!     println!("Response: {}", response.content);
//!     Ok(())
//! }
//! ```

mod client;
mod types;
mod error;

pub use client::Client;
pub use types::*;
pub use error::{Error, Result};

/// Default API endpoint.
pub const DEFAULT_ENDPOINT: &str = "http://localhost:3001";

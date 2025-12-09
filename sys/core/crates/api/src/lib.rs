//! NOA API Server
//!
//! HTTP/gRPC API endpoints for NOA services.

pub mod routes;
pub mod handlers;
pub mod server;

pub use server::Server;


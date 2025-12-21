//! NOA API server and endpoints

pub mod routes;
pub mod handlers;
pub mod server;
pub mod db;
pub mod state;

pub use server::Server;


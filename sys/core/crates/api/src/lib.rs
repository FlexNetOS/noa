//! NOA API server and endpoints

pub mod db;
pub mod handlers;
pub mod routes;
pub mod server;

pub use server::Server;

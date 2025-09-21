//! Mini Database - A Redis-like key-value store
//! 
//! This crate provides a simple, persistent key-value database with TCP networking capabilities.

pub mod database;
pub mod store;
pub mod protocol;
pub mod server;
pub mod client;

// Re-export commonly used types for convenience
pub use database::Database;
pub use store::KeyValueStore;
pub use protocol::{DatabaseCommand, DatabaseResponse};
pub use server::MiniDatabase;

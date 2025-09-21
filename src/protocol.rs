//! Protocol definitions for client-server communication

use serde::{Deserialize, Serialize};

/// Database commands sent over TCP
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseCommand {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
    Exists { key: String },
    Keys,
    Len,
    Clear,
    Ping,
}

/// Database responses sent back to clients
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseResponse {
    Ok { value: Option<String> },
    Error { message: String },
    Keys { keys: Vec<String> },
    Len { count: usize },
    Pong,
}

impl DatabaseResponse {
    pub fn success_with_value(value: String) -> Self {
        Self::Ok { value: Some(value) }
    }

    pub fn success() -> Self {
        Self::Ok { value: None }
    }

    pub fn error(message: &str) -> Self {
        Self::Error { message: message.to_string() }
    }

    pub fn keys(keys: Vec<String>) -> Self {
        Self::Keys { keys }
    }

    pub fn length(count: usize) -> Self {
        Self::Len { count }
    }

    pub fn pong() -> Self {
        Self::Pong
    }
}

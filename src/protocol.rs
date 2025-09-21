//! Protocol definitions for client-server communication
//! 
//! This module defines the command and response types used for TCP communication
//! between the database client and server.

use serde::{Deserialize, Serialize};

/// Database commands that can be sent over TCP
/// 
/// These commands represent all the operations that a client can request
/// from the database server.
/// 
/// # Examples
/// 
/// ```rust
/// use mini_db::protocol::DatabaseCommand;
/// use serde_json;
/// 
/// let cmd = DatabaseCommand::Set { 
///     key: "name".to_string(), 
///     value: "Alice".to_string() 
/// };
/// let json = serde_json::to_string(&cmd)?;
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseCommand {
    /// Get a value by key
    /// 
    /// # Fields
    /// * `key` - The key to retrieve
    Get { key: String },
    
    /// Set a key-value pair
    /// 
    /// # Fields
    /// * `key` - The key to set
    /// * `value` - The value to store
    Set { key: String, value: String },
    
    /// Delete a key-value pair
    /// 
    /// # Fields
    /// * `key` - The key to delete
    Delete { key: String },
    
    /// Check if a key exists
    /// 
    /// # Fields
    /// * `key` - The key to check
    Exists { key: String },
    
    /// List all keys in the database
    Keys,
    
    /// Get the number of key-value pairs
    Len,
    
    /// Clear all data from the database
    Clear,
    
    /// Ping the server (health check)
    Ping,
}

/// Database responses sent back to clients
/// 
/// These responses represent the results of database operations.
/// Each response type corresponds to different kinds of operations.
/// 
/// # Examples
/// 
/// ```rust
/// use mini_db::protocol::DatabaseResponse;
/// 
/// let response = DatabaseResponse::Ok { value: Some("Alice".to_string()) };
/// let error_response = DatabaseResponse::Error { message: "Key not found".to_string() };
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseResponse {
    /// Successful operation with optional value
    /// 
    /// # Fields
    /// * `value` - The result value (None for operations that don't return values)
    Ok { value: Option<String> },
    
    /// Error response
    /// 
    /// # Fields
    /// * `message` - Error description
    Error { message: String },
    
    /// Response containing a list of keys
    /// 
    /// # Fields
    /// * `keys` - Vector of all keys in the database
    Keys { keys: Vec<String> },
    
    /// Response containing the count of items
    /// 
    /// # Fields
    /// * `count` - Number of key-value pairs
    Len { count: usize },
    
    /// Pong response to ping command
    Pong,
}

impl DatabaseResponse {
    /// Create a success response with a value
    /// 
    /// # Arguments
    /// * `value` - The value to return
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::protocol::DatabaseResponse;
    /// 
    /// let response = DatabaseResponse::success_with_value("Alice");
    /// ```
    pub fn success_with_value(value: String) -> Self {
        Self::Ok { value: Some(value) }
    }

    /// Create a success response with no value
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::protocol::DatabaseResponse;
    /// 
    /// let response = DatabaseResponse::success();
    /// ```
    pub fn success() -> Self {
        Self::Ok { value: None }
    }

    /// Create an error response
    /// 
    /// # Arguments
    /// * `message` - Error message
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::protocol::DatabaseResponse;
    /// 
    /// let response = DatabaseResponse::error("Key not found");
    /// ```
    pub fn error(message: &str) -> Self {
        Self::Error { message: message.to_string() }
    }

    /// Create a keys response
    /// 
    /// # Arguments
    /// * `keys` - Vector of keys
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::protocol::DatabaseResponse;
    /// 
    /// let response = DatabaseResponse::keys(vec!["key1".to_string(), "key2".to_string()]);
    /// ```
    pub fn keys(keys: Vec<String>) -> Self {
        Self::Keys { keys }
    }

    /// Create a length response
    /// 
    /// # Arguments
    /// * `count` - Number of items
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::protocol::DatabaseResponse;
    /// 
    /// let response = DatabaseResponse::length(42);
    /// ```
    pub fn length(count: usize) -> Self {
        Self::Len { count }
    }

    /// Create a pong response
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::protocol::DatabaseResponse;
    /// 
    /// let response = DatabaseResponse::pong();
    /// ```
    pub fn pong() -> Self {
        Self::Pong
    }
}

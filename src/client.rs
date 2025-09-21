//! Database client implementation
//! 
//! This module contains the client-side code for connecting to and
//! communicating with the database server.

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;
use anyhow::Result;
use serde_json;

use crate::protocol::{DatabaseCommand, DatabaseResponse};

/// Database client for connecting to the server
/// 
/// This client handles TCP communication with the database server,
/// sending commands and receiving responses.
/// 
/// # Examples
/// 
/// ```rust
/// use mini_db::client::DatabaseClient;
/// use mini_db::protocol::DatabaseCommand;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
///     
///     // Set a value
///     client.set("name", "Alice").await?;
///     
///     // Get a value
///     if let Some(value) = client.get("name").await? {
///         println!("Got: {}", value);
///     }
///     
///     Ok(())
/// }
/// ```
pub struct DatabaseClient {
    stream: TcpStream,
}

impl DatabaseClient {
    /// Create a new database client and connect to the server
    /// 
    /// # Arguments
    /// * `addr` - Server address (e.g., "127.0.0.1:8080")
    /// 
    /// # Returns
    /// * `Result<Self>` - The connected client or an error
    /// 
    /// # Errors
    /// * Connection errors
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// ```
    pub async fn new(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        info!("Connected to database server at {}", addr);
        
        Ok(Self { stream })
    }

    /// Send a command to the server and get the response
    /// 
    /// # Arguments
    /// * `command` - The command to send
    /// 
    /// # Returns
    /// * `Result<DatabaseResponse>` - The server's response
    /// 
    /// # Errors
    /// * Network errors
    /// * JSON serialization/deserialization errors
    async fn send_command(&mut self, command: DatabaseCommand) -> Result<DatabaseResponse> {
        // Send command
        let command_data = serde_json::to_vec(&command)?;
        self.stream.write_all(&command_data).await?;

        // Read response
        let mut buffer = vec![0; 1024];
        let n = self.stream.read(&mut buffer).await?;
        let response: DatabaseResponse = serde_json::from_slice(&buffer[..n])?;

        Ok(response)
    }

    /// Get a value by key
    /// 
    /// # Arguments
    /// * `key` - The key to retrieve
    /// 
    /// # Returns
    /// * `Result<Option<String>>` - The value if found, None otherwise
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// if let Some(value) = client.get("name").await? {
    ///     println!("Found: {}", value);
    /// }
    /// ```
    pub async fn get(&mut self, key: &str) -> Result<Option<String>> {
        let response = self.send_command(DatabaseCommand::Get { key: key.to_string() }).await?;
        
        match response {
            DatabaseResponse::Ok { value } => Ok(value),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    /// Set a key-value pair
    /// 
    /// # Arguments
    /// * `key` - The key to set
    /// * `value` - The value to store
    /// 
    /// # Returns
    /// * `Result<Option<String>>` - The previous value if the key existed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// let old_value = client.set("name", "Alice").await?;
    /// ```
    pub async fn set(&mut self, key: &str, value: &str) -> Result<Option<String>> {
        let response = self.send_command(DatabaseCommand::Set { 
            key: key.to_string(), 
            value: value.to_string() 
        }).await?;
        
        match response {
            DatabaseResponse::Ok { value } => Ok(value),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    /// Delete a key-value pair
    /// 
    /// # Arguments
    /// * `key` - The key to delete
    /// 
    /// # Returns
    /// * `Result<Option<String>>` - The deleted value if the key existed
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// let deleted_value = client.delete("name").await?;
    /// ```
    pub async fn delete(&mut self, key: &str) -> Result<Option<String>> {
        let response = self.send_command(DatabaseCommand::Delete { key: key.to_string() }).await?;
        
        match response {
            DatabaseResponse::Ok { value } => Ok(value),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    /// Check if a key exists
    /// 
    /// # Arguments
    /// * `key` - The key to check
    /// 
    /// # Returns
    /// * `Result<bool>` - True if the key exists, false otherwise
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// let exists = client.exists("name").await?;
    /// ```
    pub async fn exists(&mut self, key: &str) -> Result<bool> {
        let response = self.send_command(DatabaseCommand::Exists { key: key.to_string() }).await?;
        
        match response {
            DatabaseResponse::Ok { value } => {
                match value {
                    Some(v) => Ok(v.parse().unwrap_or(false)),
                    None => Ok(false),
                }
            },
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    /// Get all keys in the database
    /// 
    /// # Returns
    /// * `Result<Vec<String>>` - Vector of all keys
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// let keys = client.keys().await?;
    /// for key in keys {
    ///     println!("Key: {}", key);
    /// }
    /// ```
    pub async fn keys(&mut self) -> Result<Vec<String>> {
        let response = self.send_command(DatabaseCommand::Keys).await?;
        
        match response {
            DatabaseResponse::Keys { keys } => Ok(keys),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    /// Get the number of key-value pairs
    /// 
    /// # Returns
    /// * `Result<usize>` - The count of items
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// let count = client.len().await?;
    /// println!("Database has {} items", count);
    /// ```
    pub async fn len(&mut self) -> Result<usize> {
        let response = self.send_command(DatabaseCommand::Len).await?;
        
        match response {
            DatabaseResponse::Len { count } => Ok(count),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    /// Clear all data from the database
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// client.clear().await?;
    /// ```
    pub async fn clear(&mut self) -> Result<()> {
        let response = self.send_command(DatabaseCommand::Clear).await?;
        
        match response {
            DatabaseResponse::Ok { .. } => Ok(()),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    /// Ping the server
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::client::DatabaseClient;
    /// 
    /// let mut client = DatabaseClient::new("127.0.0.1:8080").await?;
    /// client.ping().await?;
    /// println!("Server is alive!");
    /// ```
    pub async fn ping(&mut self) -> Result<()> {
        let response = self.send_command(DatabaseCommand::Ping).await?;
        
        match response {
            DatabaseResponse::Pong => Ok(()),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }
}

/// Run a single client command
/// 
/// This is a convenience function for running individual commands
/// without maintaining a persistent connection.
/// 
/// # Arguments
/// * `addr` - Server address
/// * `command` - The command to run
/// 
/// # Returns
/// * `Result<()>` - Success or error
/// 
/// # Examples
/// 
/// ```rust
/// use mini_db::client::run_client_command;
/// use mini_db::protocol::DatabaseCommand;
/// 
/// run_client_command("127.0.0.1:8080", DatabaseCommand::Ping).await?;
/// ```
pub async fn run_client_command(addr: &str, command: DatabaseCommand) -> Result<()> {
    let mut client = DatabaseClient::new(addr).await?;
    
    let response = client.send_command(command).await?;
    
    // Display response
    match response {
        DatabaseResponse::Ok { value } => {
            match value {
                Some(v) => println!("{}", v),
                None => println!("(null)"),
            }
        }
        DatabaseResponse::Error { message } => {
            eprintln!("Error: {}", message);
        }
        DatabaseResponse::Keys { keys } => {
            if keys.is_empty() {
                println!("(empty)");
            } else {
                for key in keys {
                    println!("{}", key);
                }
            }
        }
        DatabaseResponse::Len { count } => {
            println!("{}", count);
        }
        DatabaseResponse::Pong => {
            println!("PONG");
        }
    }

    Ok(())
}

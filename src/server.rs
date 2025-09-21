//! Database server implementation
//! 
//! This module contains the TCP server that handles client connections and processes
//! database commands.

use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tracing::{info, error, warn};
use anyhow::Result;
use serde_json;

use crate::store::KeyValueStore;
use crate::database::Database;
use crate::protocol::{DatabaseCommand, DatabaseResponse};

/// Mini database server
/// 
/// This struct manages the TCP server, handles client connections,
/// and processes database commands. It uses async/await for non-blocking I/O.
/// 
/// # Examples
/// 
/// ```rust
/// use mini_db::server::MiniDatabase;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let db = MiniDatabase::new("database.json".to_string());
///     db.start_server("127.0.0.1:8080").await?;
///     Ok(())
/// }
/// ```
pub struct MiniDatabase {
    /// Thread-safe reference to the key-value store
    store: Arc<Mutex<KeyValueStore>>,
    /// Path to the storage file
    storage_path: String,
}

impl MiniDatabase {
    /// Create a new mini database server
    /// 
    /// # Arguments
    /// * `storage_path` - Path to the JSON file for persistence
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::server::MiniDatabase;
    /// 
    /// let db = MiniDatabase::new("my-database.json".to_string());
    /// ```
    pub fn new(storage_path: String) -> Self {
        Self {
            store: Arc::new(Mutex::new(KeyValueStore::new())),
            storage_path,
        }
    }

    /// Load the database from disk
    /// 
    /// Attempts to load existing data from the storage file.
    /// If the file doesn't exist, starts with an empty database.
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    /// 
    /// # Errors
    /// * File read errors
    /// * JSON parsing errors
    async fn load_from_disk(&self) -> Result<()> {
        let store = KeyValueStore::load_from_file(&self.storage_path)?;
        *self.store.lock().await = store;
        info!("Loaded database from {}", self.storage_path);
        Ok(())
    }

    /// Save the database to disk
    /// 
    /// Persists the current state of the database to the storage file.
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    /// 
    /// # Errors
    /// * JSON serialization errors
    /// * File write errors
    async fn save_to_disk(&self) -> Result<()> {
        let store = self.store.lock().await;
        store.save_to_file(&self.storage_path)?;
        info!("Saved database to {}", self.storage_path);
        Ok(())
    }

    /// Handle a database command
    /// 
    /// Processes a command and returns the appropriate response.
    /// 
    /// # Arguments
    /// * `command` - The command to process
    /// 
    /// # Returns
    /// * `DatabaseResponse` - The response to send back to the client
    async fn handle_command(&self, command: DatabaseCommand) -> DatabaseResponse {
        let mut store = self.store.lock().await;
        
        match command {
            DatabaseCommand::Get { key } => {
                match store.get(&key) {
                    Some(value) => DatabaseResponse::success_with_value(value),
                    None => DatabaseResponse::success(),
                }
            }
            DatabaseCommand::Set { key, value } => {
                let old_value = store.set(key.clone(), value.clone());
                // Save to disk after each write
                drop(store); // Release the lock before async operation
                if let Err(e) = self.save_to_disk().await {
                    error!("Failed to save to disk: {}", e);
                }
                DatabaseResponse::Ok { value: old_value }
            }
            DatabaseCommand::Delete { key } => {
                let old_value = store.delete(&key);
                drop(store);
                if let Err(e) = self.save_to_disk().await {
                    error!("Failed to save to disk: {}", e);
                }
                DatabaseResponse::Ok { value: old_value }
            }
            DatabaseCommand::Exists { key } => {
                let exists = store.exists(&key);
                DatabaseResponse::success_with_value(exists.to_string())
            }
            DatabaseCommand::Keys => {
                let keys = store.keys();
                DatabaseResponse::keys(keys)
            }
            DatabaseCommand::Len => {
                let count = store.len();
                DatabaseResponse::length(count)
            }
            DatabaseCommand::Clear => {
                store.clear();
                drop(store);
                if let Err(e) = self.save_to_disk().await {
                    error!("Failed to save to disk: {}", e);
                }
                DatabaseResponse::success()
            }
            DatabaseCommand::Ping => {
                DatabaseResponse::pong()
            }
        }
    }

    /// Handle a client connection
    /// 
    /// Processes commands from a single client connection.
    /// Runs in a loop until the client disconnects.
    /// 
    /// # Arguments
    /// * `stream` - The TCP stream for the client connection
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    async fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
        let mut buffer = vec![0; 1024];
        
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    info!("Client disconnected");
                    break;
                }
                Ok(n) => {
                    let data = &buffer[..n];
                    
                    // Parse the command
                    let command: DatabaseCommand = match serde_json::from_slice(data) {
                        Ok(cmd) => cmd,
                        Err(e) => {
                            error!("Failed to parse command: {}", e);
                            let response = DatabaseResponse::error(&format!("Invalid command: {}", e));
                            let response_data = serde_json::to_vec(&response)?;
                            stream.write_all(&response_data).await?;
                            continue;
                        }
                    };

                    info!("Received command: {:?}", command);
                    
                    // Handle the command
                    let response = self.handle_command(command).await;
                    
                    // Send response
                    let response_data = serde_json::to_vec(&response)?;
                    stream.write_all(&response_data).await?;
                }
                Err(e) => {
                    error!("Error reading from stream: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }

    /// Start the TCP server
    /// 
    /// Binds to the specified address and starts accepting client connections.
    /// Each client connection is handled in a separate async task.
    /// 
    /// # Arguments
    /// * `addr` - The address to bind to (e.g., "127.0.0.1:8080")
    /// 
    /// # Returns
    /// * `Result<()>` - Success or error
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use mini_db::server::MiniDatabase;
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let db = MiniDatabase::new("database.json".to_string());
    ///     db.start_server("127.0.0.1:8080").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn start_server(&self, addr: &str) -> Result<()> {
        // Load existing data
        if let Err(e) = self.load_from_disk().await {
            warn!("Failed to load from disk: {}", e);
        }

        let listener = TcpListener::bind(addr).await?;
        info!("Mini database server listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    info!("New client connected: {}", addr);
                    let db = self.clone();
                    tokio::spawn(async move {
                        if let Err(e) = db.handle_client(stream).await {
                            error!("Error handling client: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

impl Clone for MiniDatabase {
    fn clone(&self) -> Self {
        Self {
            store: Arc::clone(&self.store),
            storage_path: self.storage_path.clone(),
        }
    }
}

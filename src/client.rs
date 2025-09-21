//! Database client implementation

use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;
use anyhow::Result;
use serde_json;

use crate::protocol::{DatabaseCommand, DatabaseResponse};

/// Database client for connecting to the server
pub struct DatabaseClient {
    stream: TcpStream,
}

impl DatabaseClient {
    pub async fn new(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        info!("Connected to database server at {}", addr);
        
        Ok(Self { stream })
    }

    async fn send_command(&mut self, command: DatabaseCommand) -> Result<DatabaseResponse> {
        let command_data = serde_json::to_vec(&command)?;
        self.stream.write_all(&command_data).await?;

        let mut buffer = vec![0; 1024];
        let n = self.stream.read(&mut buffer).await?;
        let response: DatabaseResponse = serde_json::from_slice(&buffer[..n])?;

        Ok(response)
    }

    pub async fn get(&mut self, key: &str) -> Result<Option<String>> {
        let response = self.send_command(DatabaseCommand::Get { key: key.to_string() }).await?;
        
        match response {
            DatabaseResponse::Ok { value } => Ok(value),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

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

    pub async fn delete(&mut self, key: &str) -> Result<Option<String>> {
        let response = self.send_command(DatabaseCommand::Delete { key: key.to_string() }).await?;
        
        match response {
            DatabaseResponse::Ok { value } => Ok(value),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

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

    pub async fn keys(&mut self) -> Result<Vec<String>> {
        let response = self.send_command(DatabaseCommand::Keys).await?;
        
        match response {
            DatabaseResponse::Keys { keys } => Ok(keys),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    pub async fn len(&mut self) -> Result<usize> {
        let response = self.send_command(DatabaseCommand::Len).await?;
        
        match response {
            DatabaseResponse::Len { count } => Ok(count),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

    pub async fn clear(&mut self) -> Result<()> {
        let response = self.send_command(DatabaseCommand::Clear).await?;
        
        match response {
            DatabaseResponse::Ok { .. } => Ok(()),
            DatabaseResponse::Error { message } => Err(anyhow::anyhow!("Server error: {}", message)),
            _ => Err(anyhow::anyhow!("Unexpected response type")),
        }
    }

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
pub async fn run_client_command(addr: &str, command: DatabaseCommand) -> Result<()> {
    let mut client = DatabaseClient::new(addr).await?;
    
    let response = client.send_command(command).await?;
    
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

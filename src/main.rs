//! Mini Database CLI
//! 
//! This is the main entry point for the mini database application.
//! It provides both server and client functionality through command-line interface.

use clap::{Parser, Subcommand};
use tracing_subscriber;
use anyhow::Result;

use kv_store::protocol::{DatabaseCommand};
use kv_store::server::MiniDatabase;
use kv_store::client::run_client_command;

#[derive(Parser)]
#[command(name = "mini-db")]
#[command(about = "A mini database server (like a tiny Redis)")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the TCP server
    Server {
        /// Address to bind to
        #[arg(long, default_value = "127.0.0.1:8080")]
        addr: String,
        /// Storage file path
        #[arg(long, default_value = "mini-db.json")]
        storage: String,
    },
    /// Run a client command
    Client {
        /// Server address
        #[arg(long, default_value = "127.0.0.1:8080")]
        addr: String,
        #[command(subcommand)]
        command: ClientCommands,
    },
}

#[derive(Subcommand)]
enum ClientCommands {
    /// Get a value by key
    Get { 
        /// The key to retrieve
        key: String 
    },
    /// Set a key-value pair
    Set { 
        /// The key to set
        key: String, 
        /// The value to store
        value: String 
    },
    /// Delete a key
    Delete { 
        /// The key to delete
        key: String 
    },
    /// Check if key exists
    Exists { 
        /// The key to check
        key: String 
    },
    /// List all keys
    Keys,
    /// Get the number of keys
    Len,
    /// Clear all data
    Clear,
    /// Ping the server
    Ping,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Server { addr, storage } => {
            println!("🚀 Starting mini database server...");
            println!("📡 Listening on: {}", addr);
            println!("💾 Storage file: {}", storage);
            println!("📝 Logs will appear below:");
            println!();
            
            let db = MiniDatabase::new(storage);
            db.start_server(&addr).await?;
        }
        Commands::Client { addr, command } => {
            let db_command = match command {
                ClientCommands::Get { key } => DatabaseCommand::Get { key },
                ClientCommands::Set { key, value } => DatabaseCommand::Set { key, value },
                ClientCommands::Delete { key } => DatabaseCommand::Delete { key },
                ClientCommands::Exists { key } => DatabaseCommand::Exists { key },
                ClientCommands::Keys => DatabaseCommand::Keys,
                ClientCommands::Len => DatabaseCommand::Len,
                ClientCommands::Clear => DatabaseCommand::Clear,
                ClientCommands::Ping => DatabaseCommand::Ping,
            };

            run_client_command(&addr, db_command).await?;
        }
    }

    Ok(())
}
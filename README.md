# 🗄️ Mini Database - A Redis-like Key-Value Store

A comprehensive Rust learning project that implements a mini database server with TCP networking, persistence, and concurrent client support. This project demonstrates advanced Rust concepts including async programming, traits, generics, error handling, and thread safety.

## 🎯 Project Overview

This mini database is designed as a **learning laboratory** for Rust concepts. It implements a Redis-like key-value store with:

- **TCP Server**: Handles multiple concurrent clients
- **Persistence**: Automatic saving to JSON files
- **Thread Safety**: Safe concurrent access using `Arc<Mutex<T>>`
- **Async I/O**: Non-blocking operations with Tokio
- **CLI Interface**: Professional command-line interface
- **Modular Design**: Clean separation of concerns

## 🏗️ Architecture

```
┌─────────────────┐    TCP/JSON    ┌─────────────────┐
│   CLI Client    │◄──────────────►│  Database       │
│                 │                │  Server         │
└─────────────────┘                └─────────────────┘
                                           │
                                           ▼
                                    ┌─────────────────┐
                                    │  KeyValueStore  │
                                    │  (HashMap)      │
                                    └─────────────────┘
                                           │
                                           ▼
                                    ┌─────────────────┐
                                    │  JSON File      │
                                    │  (Persistence)  │
                                    └─────────────────┘
```

## 📁 Project Structure

```
src/
├── lib.rs          # Library root with module declarations
├── main.rs         # CLI entry point and command orchestration
├── database.rs     # Database trait definition (interface)
├── store.rs        # KeyValueStore implementation (concrete)
├── protocol.rs     # Command/Response types for TCP communication
├── server.rs       # TCP server with async client handling
└── client.rs       # Client implementation for connecting to server
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (with Cargo)
- Basic understanding of command-line interfaces

### Installation

1. Clone or download the project
2. Navigate to the project directory
3. Build the project:

```bash
cargo build
```

### Running the Server

Start the database server:

```bash
cargo run -- server --addr 127.0.0.1:8080 --storage my-database.json
```

You should see:
```
🚀 Starting mini database server...
📡 Listening on: 127.0.0.1:8080
💾 Storage file: my-database.json
📝 Logs will appear below:

2024-01-01T12:00:00Z INFO mini_database server listening on 127.0.0.1:8080
```

### Using the Client

In a **new terminal**, run client commands:

```bash
# Ping the server
cargo run -- client --addr 127.0.0.1:8080 ping

# Set a key-value pair
cargo run -- client --addr 127.0.0.1:8080 set "name" "Alice"

# Get a value
cargo run -- client --addr 127.0.0.1:8080 get "name"

# List all keys
cargo run -- client --addr 127.0.0.1:8080 keys

# Get count of items
cargo run -- client --addr 127.0.0.1:8080 len

# Delete a key
cargo run -- client --addr 127.0.0.1:8080 delete "name"

# Clear all data
cargo run -- client --addr 127.0.0.1:8080 clear
```

## 📚 Available Commands

### Server Commands

```bash
cargo run -- server [OPTIONS]

Options:
  --addr <ADDR>        Address to bind to [default: 127.0.0.1:8080]
  --storage <STORAGE>  Storage file path [default: mini-db.json]
  -h, --help           Print help
```

### Client Commands

```bash
cargo run -- client --addr <ADDR> <COMMAND>

Commands:
  get <KEY>        Get a value by key
  set <KEY> <VALUE> Set a key-value pair
  delete <KEY>     Delete a key
  exists <KEY>     Check if key exists
  keys            List all keys
  len             Get the number of keys
  clear           Clear all data
  ping            Ping the server
```

## 🔧 Technical Details

### Core Rust Concepts Demonstrated

#### 1. **Traits and Polymorphism**
```rust
trait Database<K, V> {
    fn get(&self, key: &K) -> Option<V>;
    fn set(&mut self, key: K, value: V) -> Option<V>;
    // ... more methods
}
```

#### 2. **Async Programming with Tokio**
```rust
async fn handle_client(&self, mut stream: TcpStream) -> Result<()> {
    let mut buffer = vec![0; 1024];
    match stream.read(&mut buffer).await {
        Ok(n) => { /* process data */ }
        Err(e) => { /* handle error */ }
    }
}
```

#### 3. **Thread Safety with Arc<Mutex<T>>**
```rust
store: Arc<Mutex<KeyValueStore>>,

// Usage
let mut store = self.store.lock().await;
store.set(key, value);
drop(store); // Release lock explicitly
```

#### 4. **Error Handling with Result<T, E>**
```rust
fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
    let contents = fs::read_to_string(path)
        .context("Failed to read file")?;
    let store: Self = serde_json::from_str(&contents)
        .context("Failed to parse JSON")?;
    Ok(store)
}
```

#### 5. **Enums with Data and Pattern Matching**
```rust
enum DatabaseCommand {
    Get { key: String },
    Set { key: String, value: String },
    Delete { key: String },
}

match command {
    DatabaseCommand::Get { key } => { /* handle get */ }
    DatabaseCommand::Set { key, value } => { /* handle set */ }
    // Must handle all cases!
}
```

#### 6. **Serialization with Serde**
```rust
#[derive(Serialize, Deserialize)]
struct KeyValueStore {
    data: HashMap<String, String>,
    created_at: u64,
    updated_at: u64,
}
```

### Communication Protocol

The client and server communicate using JSON over TCP:

**Command Example:**
```json
{
  "Set": {
    "key": "name",
    "value": "Alice"
  }
}
```

**Response Example:**
```json
{
  "Ok": {
    "value": null
  }
}
```

### Concurrency Model

- **Server**: Handles multiple clients concurrently using `tokio::spawn`
- **Thread Safety**: Uses `Arc<Mutex<KeyValueStore>>` for safe shared access
- **Non-blocking I/O**: Async operations don't block other clients
- **Lock Management**: Explicit lock release to prevent deadlocks

## 🧪 Testing the System

### Basic Functionality Test

1. **Start the server**:
   ```bash
   cargo run -- server --addr 127.0.0.1:8080 --storage test-db.json
   ```

2. **Test basic operations**:
   ```bash
   # Set data
   cargo run -- client --addr 127.0.0.1:8080 set "name" "Alice"
   cargo run -- client --addr 127.0.0.1:8080 set "age" "25"
   
   # Retrieve data
   cargo run -- client --addr 127.0.0.1:8080 get "name"
   cargo run -- client --addr 127.0.0.1:8080 get "age"
   
   # List keys
   cargo run -- client --addr 127.0.0.1:8080 keys
   
   # Get count
   cargo run -- client --addr 127.0.0.1:8080 len
   ```

3. **Test persistence**:
   - Stop the server (Ctrl+C)
   - Restart the server
   - Check if data persisted: `cargo run -- client --addr 127.0.0.1:8080 keys`

### Concurrent Access Test

Open multiple terminals and run client commands simultaneously to test thread safety.

### Error Handling Test

```bash
# Try to get non-existent key
cargo run -- client --addr 127.0.0.1:8080 get "nonexistent"

# Try to delete non-existent key
cargo run -- client --addr 127.0.0.1:8080 delete "nonexistent"
```

## 📖 Learning Objectives

This project teaches:

### **Rust Fundamentals**
- Ownership and borrowing
- Traits and implementations
- Enums and pattern matching
- Error handling with `Result<T, E>`
- Generics and trait bounds

### **Advanced Rust Concepts**
- Async/await programming
- Concurrency with `Arc<Mutex<T>>`
- Network programming
- Serialization/deserialization
- Command-line interface design

### **Software Architecture**
- Modular design principles
- Separation of concerns
- Interface design with traits
- Error-first design patterns
- Professional project structure

### **Systems Programming**
- TCP networking
- File I/O and persistence
- Concurrent server design
- Memory management
- Performance considerations

## 🔍 Code Walkthrough

### Module-by-Module Explanation

1. **`database.rs`**: Defines the `Database` trait - the interface that all storage backends must implement
2. **`store.rs`**: Concrete implementation using `HashMap` with JSON persistence
3. **`protocol.rs`**: Defines `DatabaseCommand` and `DatabaseResponse` enums for TCP communication
4. **`server.rs`**: TCP server that handles multiple clients concurrently using async tasks
5. **`client.rs`**: Client implementation that connects to the server and sends commands
6. **`main.rs`**: CLI interface that orchestrates server and client functionality
7. **`lib.rs`**: Library root that exports public APIs

### Key Design Patterns

- **Trait-based design**: Easy to swap storage backends
- **Enum-based protocols**: Type-safe command/response handling
- **Async task spawning**: Concurrent client handling
- **Explicit error handling**: No panics, graceful error recovery
- **Modular architecture**: Clear separation of concerns

## 🚀 Extensions and Improvements

### Possible Enhancements

1. **Additional Storage Backends**:
   - Redis backend
   - SQLite backend
   - In-memory with TTL

2. **Advanced Features**:
   - Key expiration
   - Transactions
   - Pub/Sub messaging
   - Clustering support

3. **Performance Optimizations**:
   - Connection pooling
   - Caching layer
   - Compression
   - Binary protocol

4. **Monitoring and Observability**:
   - Metrics collection
   - Health checks
   - Performance monitoring
   - Distributed tracing

## 🤝 Contributing

This is a learning project, but contributions are welcome! Areas for improvement:

- Additional test cases
- Performance benchmarks
- Documentation improvements
- New features
- Bug fixes

## 📄 License

This project is for educational purposes. Feel free to use and modify for learning Rust!

## 🎓 Educational Value

This project serves as a comprehensive introduction to:

- **Modern Rust development**
- **Systems programming concepts**
- **Network programming**
- **Concurrent programming**
- **Professional software architecture**

It demonstrates how Rust's type system, ownership model, and async runtime enable building robust, performant, and safe systems software.

---

**Happy Learning! 🦀**

*This mini database project showcases the power and elegance of Rust for building real-world applications while maintaining memory safety, performance, and developer productivity.*

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
- Make (for convenient commands)
- Basic understanding of command-line interfaces

### Installation

1. Clone or download the project
2. Navigate to the project directory
3. Build the project:

```bash
make build
# or
cargo build
```

### Running the Server

Start the database server using Make:

```bash
make run-server
# or with custom settings
make run-server SERVER_ADDR=127.0.0.1:8080 STORAGE_FILE=my-database.json
```

You should see:
```
🚀 Starting mini database server...
📡 Address: 127.0.0.1:8080
💾 Storage: mini-db.json
📝 Press Ctrl+C to stop

2024-01-01T12:00:00Z INFO mini_database server listening on 127.0.0.1:8080
```

### Using the Client

In a **new terminal**, run client commands using Make:

```bash
# Ping the server
make ping

# Set a key-value pair
make set KEY=name VALUE=Alice

# Get a value
make get KEY=name

# List all keys
make keys

# Get count of items
make len

# Delete a key
make delete KEY=name

# Clear all data
make clear
```

### Available Make Commands

Run `make help` to see all available commands:

```bash
make help
```

**Build Commands:**
- `make build` - Build the project
- `make check` - Check the project without building
- `make clean` - Clean build artifacts

**Server Commands:**
- `make run-server` - Start the database server
- `make run-server-dev` - Start server with development settings

**Client Commands:**
- `make ping` - Ping the server
- `make set KEY=name VALUE=Alice` - Set a key-value pair
- `make get KEY=name` - Get a value by key
- `make delete KEY=name` - Delete a key
- `make keys` - List all keys
- `make len` - Get count of items
- `make clear` - Clear all data

**Testing Commands:**
- `make test` - Run all tests
- `make test-basic` - Run basic functionality test
- `make test-concurrent` - Run concurrent access test
- `make demo` - Run a complete demo

**Development Commands:**
- `make fmt` - Format the code
- `make clippy` - Run clippy linter
- `make dev-setup` - Set up development environment

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

### Quick Testing with Make

The easiest way to test the system is using the provided Make commands:

```bash
# Run a complete demo
make demo

# Run basic functionality test
make test-basic

# Run concurrent access test
make test-concurrent

# Run all tests
make test
```

### Manual Testing

#### Basic Functionality Test

1. **Start the server**:
   ```bash
   make run-server
   # or
   cargo run -- server --addr 127.0.0.1:8080 --storage test-db.json
   ```

2. **Test basic operations**:
   ```bash
   # Using Make (recommended)
   make set KEY=name VALUE=Alice
   make set KEY=age VALUE=25
   make get KEY=name
   make get KEY=age
   make keys
   make len
   
   # Or using cargo directly
   cargo run -- client --addr 127.0.0.1:8080 set "name" "Alice"
   cargo run -- client --addr 127.0.0.1:8080 set "age" "25"
   cargo run -- client --addr 127.0.0.1:8080 get "name"
   cargo run -- client --addr 127.0.0.1:8080 get "age"
   cargo run -- client --addr 127.0.0.1:8080 keys
   cargo run -- client --addr 127.0.0.1:8080 len
   ```

3. **Test persistence**:
   - Stop the server (Ctrl+C)
   - Restart the server
   - Check if data persisted: `make keys`

### Concurrent Access Test

Open multiple terminals and run client commands simultaneously to test thread safety:

```bash
# Terminal 1
make set KEY=client1 VALUE=data1

# Terminal 2  
make set KEY=client2 VALUE=data2

# Terminal 3
make set KEY=client3 VALUE=data3
```

### Error Handling Test

```bash
# Try to get non-existent key
make get KEY=nonexistent

# Try to delete non-existent key
make delete KEY=nonexistent
```

## 🛠️ Makefile Features

The project includes a comprehensive Makefile that provides convenient commands for:

### **Build Management**
- `make build` - Build the project
- `make check` - Check without building
- `make clean` - Clean artifacts and storage files
- `make release` - Build optimized release version

### **Development Workflow**
- `make fmt` - Format code with rustfmt
- `make clippy` - Run clippy linter
- `make dev-setup` - Set up development environment
- `make info` - Show project information

### **Server Management**
- `make run-server` - Start server with default settings
- `make run-server-dev` - Start server with debug logging
- `make stop` - Stop running server processes

### **Client Operations**
- `make ping` - Ping the server
- `make set KEY=name VALUE=Alice` - Set key-value pair
- `make get KEY=name` - Get value by key
- `make delete KEY=name` - Delete key
- `make keys` - List all keys
- `make len` - Get item count
- `make clear` - Clear all data

### **Automated Testing**
- `make test` - Run all unit tests
- `make test-basic` - Automated basic functionality test
- `make test-concurrent` - Automated concurrent access test
- `make demo` - Complete feature demonstration

### **Configuration**
You can customize the Makefile behavior with environment variables:

```bash
# Custom server address
make run-server SERVER_ADDR=0.0.0.0:9090

# Custom storage file
make run-server STORAGE_FILE=production-db.json

# Custom settings for client commands
make get KEY=name SERVER_ADDR=127.0.0.1:9090
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

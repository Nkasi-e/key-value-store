

.PHONY: help build run-server run-client test clean install check fmt clippy

help:
	@echo "Mini Database - Available Commands:"
	@echo ""
	@echo "Build Commands:"
	@echo "  build          Build the project"
	@echo "  check          Check the project without building"
	@echo "  clean          Clean build artifacts"
	@echo ""
	@echo "Development Commands:"
	@echo "  fmt            Format the code"
	@echo "  clippy         Run clippy linter"
	@echo "  install        Install dependencies"
	@echo ""
	@echo "Server Commands:"
	@echo "  run-server     Start the database server"
	@echo "  run-server-dev Start server with development settings"
	@echo ""
	@echo "Client Commands:"
	@echo "  ping           Ping the server"
	@echo "  set            Set a key-value pair (usage: make set KEY=name VALUE=Alice)"
	@echo "  get            Get a value by key (usage: make get KEY=name)"
	@echo "  delete         Delete a key (usage: make delete KEY=name)"
	@echo "  keys           List all keys"
	@echo "  len            Get count of items"
	@echo "  clear          Clear all data"
	@echo ""
	@echo "Testing Commands:"
	@echo "  test           Run all tests"
	@echo "  test-basic     Run basic functionality test"
	@echo "  test-concurrent Run concurrent access test"
	@echo "  demo           Run a complete demo"
	@echo ""
	@echo "Configuration:"
	@echo "  SERVER_ADDR    Server address (default: 127.0.0.1:8080)"
	@echo "  STORAGE_FILE   Storage file path (default: mini-db.json)"

# Configuration variables
SERVER_ADDR ?= 127.0.0.1:8080
STORAGE_FILE ?= mini-db.json
KEY ?= 
VALUE ?= 

# Build Commands
build:
	@echo "🔨 Building mini database..."
	cargo build

check:
	@echo "🔍 Checking project..."
	cargo check

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@echo "🗑️  Removing storage files..."
	rm -f *.json

install:
	@echo "📦 Installing dependencies..."
	cargo build

# Development Commands
fmt:
	@echo "🎨 Formatting code..."
	cargo fmt

clippy:
	@echo "🔍 Running clippy linter..."
	cargo clippy -- -D warnings

# Server Commands
run-server:
	@echo "🚀 Starting mini database server..."
	@echo "📡 Address: $(SERVER_ADDR)"
	@echo "💾 Storage: $(STORAGE_FILE)"
	@echo "📝 Press Ctrl+C to stop"
	@echo ""
	cargo run -- server --addr $(SERVER_ADDR) --storage $(STORAGE_FILE)

run-server-dev:
	@echo "🚀 Starting mini database server (development mode)..."
	@echo "📡 Address: $(SERVER_ADDR)"
	@echo "💾 Storage: $(STORAGE_FILE)"
	@echo "🔧 Development settings enabled"
	@echo "📝 Press Ctrl+C to stop"
	@echo ""
	RUST_LOG=debug cargo run -- server --addr $(SERVER_ADDR) --storage $(STORAGE_FILE)

# Client Commands
ping:
	@echo "🏓 Pinging server at $(SERVER_ADDR)..."
	cargo run -- client --addr $(SERVER_ADDR) ping

set:
	@if [ -z "$(KEY)" ] || [ -z "$(VALUE)" ]; then \
		echo "❌ Error: Please provide KEY and VALUE"; \
		echo "Usage: make set KEY=name VALUE=Alice"; \
		exit 1; \
	fi
	@echo "📝 Setting key '$(KEY)' to '$(VALUE)'..."
	cargo run -- client --addr $(SERVER_ADDR) set "$(KEY)" "$(VALUE)"

get:
	@if [ -z "$(KEY)" ]; then \
		echo "❌ Error: Please provide KEY"; \
		echo "Usage: make get KEY=name"; \
		exit 1; \
	fi
	@echo "🔍 Getting value for key '$(KEY)'..."
	cargo run -- client --addr $(SERVER_ADDR) get "$(KEY)"

delete:
	@if [ -z "$(KEY)" ]; then \
		echo "❌ Error: Please provide KEY"; \
		echo "Usage: make delete KEY=name"; \
		exit 1; \
	fi
	@echo "🗑️  Deleting key '$(KEY)'..."
	cargo run -- client --addr $(SERVER_ADDR) delete "$(KEY)"

keys:
	@echo "📋 Listing all keys..."
	cargo run -- client --addr $(SERVER_ADDR) keys

len:
	@echo "📊 Getting count of items..."
	cargo run -- client --addr $(SERVER_ADDR) len

clear:
	@echo "🧹 Clearing all data..."
	cargo run -- client --addr $(SERVER_ADDR) clear

# Testing Commands
test:
	@echo "🧪 Running all tests..."
	cargo test

test-basic:
	@echo "🧪 Running basic functionality test..."
	@echo "This test will start a server, run basic operations, and stop it."
	@echo ""
	@echo "Starting server in background..."
	@cargo run -- server --addr $(SERVER_ADDR) --storage test-basic.json > /dev/null 2>&1 & \
	SERVER_PID=$$!; \
	sleep 2; \
	echo "Testing basic operations..."; \
	make set KEY=name VALUE=Alice SERVER_ADDR=$(SERVER_ADDR) > /dev/null; \
	make set KEY=age VALUE=25 SERVER_ADDR=$(SERVER_ADDR) > /dev/null; \
	make get KEY=name SERVER_ADDR=$(SERVER_ADDR); \
	make keys SERVER_ADDR=$(SERVER_ADDR); \
	make len SERVER_ADDR=$(SERVER_ADDR); \
	make delete KEY=age SERVER_ADDR=$(SERVER_ADDR); \
	make get KEY=age SERVER_ADDR=$(SERVER_ADDR); \
	make clear SERVER_ADDR=$(SERVER_ADDR); \
	echo "Stopping server..."; \
	kill $$SERVER_PID 2>/dev/null || true; \
	rm -f test-basic.json; \
	echo "✅ Basic test completed!"

test-concurrent:
	@echo "🧪 Running concurrent access test..."
	@echo "This test will start a server and run multiple clients simultaneously."
	@echo ""
	@echo "Starting server in background..."
	@cargo run -- server --addr $(SERVER_ADDR) --storage test-concurrent.json > /dev/null 2>&1 & \
	SERVER_PID=$$!; \
	sleep 2; \
	echo "Running concurrent clients..."; \
	make set KEY=client1 VALUE=data1 SERVER_ADDR=$(SERVER_ADDR) & \
	make set KEY=client2 VALUE=data2 SERVER_ADDR=$(SERVER_ADDR) & \
	make set KEY=client3 VALUE=data3 SERVER_ADDR=$(SERVER_ADDR) & \
	wait; \
	make keys SERVER_ADDR=$(SERVER_ADDR); \
	make len SERVER_ADDR=$(SERVER_ADDR); \
	echo "Stopping server..."; \
	kill $$SERVER_PID 2>/dev/null || true; \
	rm -f test-concurrent.json; \
	echo "✅ Concurrent test completed!"

demo:
	@echo "🎬 Running complete demo..."
	@echo "This demo will showcase all features of the mini database."
	@echo ""
	@echo "Starting server in background..."
	@cargo run -- server --addr $(SERVER_ADDR) --storage demo.json > /dev/null 2>&1 & \
	SERVER_PID=$$!; \
	sleep 2; \
	echo ""; \
	echo "=== Mini Database Demo ==="; \
	echo ""; \
	echo "1. Testing ping..."; \
	make ping SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "2. Setting some data..."; \
	make set KEY=name VALUE=Alice SERVER_ADDR=$(SERVER_ADDR); \
	make set KEY=age VALUE=25 SERVER_ADDR=$(SERVER_ADDR); \
	make set KEY=city VALUE=New\ York SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "3. Retrieving data..."; \
	make get KEY=name SERVER_ADDR=$(SERVER_ADDR); \
	make get KEY=age SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "4. Listing all keys..."; \
	make keys SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "5. Getting count..."; \
	make len SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "6. Deleting a key..."; \
	make delete KEY=age SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "7. Checking deletion..."; \
	make get KEY=age SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "8. Final state..."; \
	make keys SERVER_ADDR=$(SERVER_ADDR); \
	make len SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "9. Clearing all data..."; \
	make clear SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "10. Final verification..."; \
	make len SERVER_ADDR=$(SERVER_ADDR); \
	echo ""; \
	echo "Stopping server..."; \
	kill $$SERVER_PID 2>/dev/null || true; \
	rm -f demo.json; \
	echo "🎉 Demo completed successfully!"

# Convenience aliases
server: run-server
client: ping
start: run-server
stop:
	@echo "🛑 Stopping server..."
	@pkill -f "cargo run -- server" || echo "No server process found"

# Development workflow
dev-setup: install fmt clippy
	@echo "✅ Development environment ready!"

# Production build
release:
	@echo "🚀 Building release version..."
	cargo build --release

# Show project info
info:
	@echo "Mini Database Project Information:"
	@echo "=================================="
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "Project location: $$(pwd)"
	@echo "Server address: $(SERVER_ADDR)"
	@echo "Storage file: $(STORAGE_FILE)"
	@echo ""
	@echo "Available commands: make help"

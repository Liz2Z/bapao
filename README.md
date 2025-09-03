# 基于 Gitee 的内外网通信系统

A Gitee-based internal-external network communication system that enables secure communication between networks through Gitee repositories.

## 📚 Documentation

**Complete documentation is available in the `docs/` directory:**

- **[📋 Start Here - Documentation Index](./docs/index.md)** - Main navigation and overview
- **[🚀 Quick Start Guide](./docs/README.md)** - Project introduction and setup
- **[⚙️ Configuration Guide](./docs/configuration.md)** - Setup instructions
- **[📖 API Reference](./docs/api_reference.md)** - Complete API documentation
- **[💡 Examples & Patterns](./docs/examples.md)** - Code examples and usage
- **[🔌 Application Protocol](./docs/app_protocol_api.md)** - High-level API docs
- **[🚚 Transport Protocol](./docs/transport_protocol_api.md)** - Low-level transport docs

## Quick Setup

### 1. Configuration

Create `bapao.config.json`:

```json
{
  "access_token": "your_gitee_access_token",
  "user_name": "your_gitee_username",
  "repo": "your_repository_name", 
  "file_path": "io"
}
```

### 2. Build and Run

```bash
cargo build --release
cargo run
```

### 3. Usage Example

```rust
use bapao_app_protocal::{AppListener, TransUnitType};

fn hello() -> TransUnitType {
    TransUnitType::String("Hello from Bapao!".to_string())
}

#[tokio::main]
async fn main() {
    let mut listener = AppListener::new();
    listener.add("/hello", hello);
    listener.listen().await;
}
```

## Features

- 🔐 **Secure Communication** through private Gitee repositories
- 📷 **Screenshot Capture** for remote monitoring
- 📁 **File Transfer** support for binary and text data
- 🔄 **Request/Response** structured communication pattern
- ⏰ **Automatic Cleanup** of expired requests (30min timeout)
- 🚀 **Async/Await** throughout for performance

**[📖 See full documentation for detailed guides and examples →](./docs/index.md)**

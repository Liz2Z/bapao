# Bapao Communication System - Complete Documentation

## Quick Navigation

| Document | Description |
|----------|-------------|
| [📋 Project Overview](./README.md) | Project introduction, architecture, and quick start |
| [🔌 Application Protocol API](./app_protocol_api.md) | High-level API for building applications |
| [🚀 Transport Protocol API](./transport_protocol_api.md) | Low-level transport and Gitee integration |
| [⚙️ Configuration Guide](./configuration.md) | Setup and configuration instructions |
| [📚 Complete API Reference](./api_reference.md) | Detailed function signatures and examples |
| [💡 Examples & Usage](./examples.md) | Code examples and usage patterns |
| [🎯 Main Application](./main_application.md) | Documentation for the included screenshot app |

---

## What is Bapao?

Bapao is a **Gitee-based internal-external network communication system** that enables secure communication between networks that cannot directly connect to each other. It uses Gitee repositories as a communication channel, allowing:

- **Remote Command Execution**: Execute commands on internal systems from external networks
- **File Transfer**: Transfer files bidirectionally through the repository
- **System Monitoring**: Monitor internal systems from external locations
- **API Services**: Provide API endpoints accessible through the repository

---

## Getting Started

### 1. Quick Setup

```bash
# Clone and build
git clone <repository-url>
cd bapao-system
cargo build --release

# Configure
cp bapao.config.json.example bapao.config.json
# Edit bapao.config.json with your Gitee details

# Run
cargo run
```

### 2. Basic Usage

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

### 3. Send Request

Update your Gitee repository file with:
```json
[{
  "head": {"id": "1", "state": "Pending", "timestamp": 1704067200000},
  "body": "/hello"
}]
```

---

## Architecture Overview

```
External Network          Gitee Repository          Internal Network
┌─────────────────┐      ┌─────────────────┐      ┌─────────────────┐
│                 │      │                 │      │                 │
│  Client App     │◄────►│  JSON File      │◄────►│  Bapao App      │
│                 │      │  (Channel)      │      │                 │
│  - Send Requests│      │  - Requests     │      │  - Process      │
│  - Get Responses│      │  - Responses    │      │  - Respond      │
│                 │      │  - File Storage │      │  - File Upload  │
└─────────────────┘      └─────────────────┘      └─────────────────┘
```

---

## Core Concepts

### Request/Response Cycle

1. **External Client** writes request to Gitee repository file
2. **Bapao Application** polls repository every 10 seconds
3. **Request Processing** happens through registered route handlers
4. **Response** is written back to repository
5. **External Client** reads response from repository

### Data Types

- **Text Responses**: JSON strings, plain text, structured data
- **File Responses**: Images, documents, binary data
- **Request Routing**: URL-style paths for different services

### State Management

- **Pending**: New requests awaiting processing
- **Done**: Completed requests with responses
- **Expired**: Requests older than 30 minutes (automatically cleaned)

---

## Key Features

### 🔐 Security
- Private repository communication
- Access token authentication
- Request expiration (30min timeout)
- Path traversal protection

### 📊 Monitoring
- Built-in screenshot capture
- System information endpoints
- Health check capabilities
- Request/response logging

### 📁 File Transfer
- Binary file support
- Automatic base64 encoding
- UUID-based file naming
- Large file handling

### ⚡ Performance
- Async/await throughout
- Efficient polling mechanism
- Memory-conscious file handling
- Automatic cleanup

---

## Use Cases

### Remote System Administration
```rust
listener.add("/system/restart", system_restart_handler);
listener.add("/system/logs", get_system_logs);
listener.add("/system/status", get_system_status);
```

### IoT Device Monitoring
```rust
listener.add("/sensors/temperature", read_temperature);
listener.add("/sensors/humidity", read_humidity);
listener.add("/camera/capture", capture_image);
```

### File Synchronization
```rust
listener.add("/sync/upload", handle_file_upload);
listener.add("/sync/download", handle_file_download);
listener.add("/sync/list", list_available_files);
```

### Remote Development
```rust
listener.add("/dev/build", trigger_build);
listener.add("/dev/test", run_tests);
listener.add("/dev/deploy", deploy_application);
```

---

## Best Practices

### 1. Error Handling
- Always return meaningful error messages
- Use structured JSON for error responses
- Implement fallback mechanisms
- Log errors for debugging

### 2. Security
- Validate all inputs
- Use private repositories
- Implement access controls
- Monitor for suspicious activity

### 3. Performance
- Limit response sizes
- Use caching where appropriate
- Clean up temporary files
- Monitor memory usage

### 4. Reliability
- Implement health checks
- Use proper error recovery
- Handle network failures gracefully
- Test edge cases

---

## Troubleshooting Guide

### Common Problems

| Problem | Cause | Solution |
|---------|-------|----------|
| "No config file found" | Missing `bapao.config.json` | Create config file in project root |
| "Authentication failed" | Invalid access token | Check token permissions and validity |
| "No responses received" | Network/repository issues | Verify repository access and network |
| "Screenshot command failed" | Missing screenshot tools | Install `scrot`, `gnome-screenshot`, or platform equivalent |

### Debug Commands

```bash
# Check configuration
cat bapao.config.json

# Test Gitee API access
curl -H "Authorization: token YOUR_TOKEN" \
  "https://gitee.com/api/v5/repos/USER/REPO/contents/FILE"

# Run with debug logging
RUST_LOG=debug cargo run

# Check system dependencies
which scrot gnome-screenshot screencapture
```

---

## Next Steps

1. **Read the [Configuration Guide](./configuration.md)** to set up your environment
2. **Explore [Examples](./examples.md)** to understand usage patterns
3. **Check the [API Reference](./api_reference.md)** for detailed function documentation
4. **Review [Security Best Practices](#security)** before deploying to production

---

## Support and Contributing

- **Issues**: Report bugs and feature requests in the repository
- **Documentation**: Suggest improvements to documentation
- **Examples**: Contribute additional usage examples
- **Security**: Report security issues privately

---

*This documentation covers version 0.1.0 of the Bapao communication system.*
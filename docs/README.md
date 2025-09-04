# Bapao Communication System Documentation

## Overview

Bapao is a Gitee-based internal-external network communication system that enables secure communication between internal and external networks using Gitee repositories as a communication channel.

## Project Structure

This project consists of three main components:

- **`app/`** - Main application that handles requests and responses
- **`bapao_app_protocal/`** - Application protocol layer for handling requests
- **`bapao_trans_protocal/`** - Transport protocol layer for Gitee communication
- **`utils/`** - Shared utilities

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   External      │    │     Gitee        │    │   Internal      │
│   Client        │◄──►│   Repository     │◄──►│   Bapao App     │
│                 │    │   (Channel)      │    │                 │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

The system works by:
1. External clients send requests by updating a file in a Gitee repository
2. The internal Bapao application polls the repository for new requests
3. Requests are processed and responses are written back to the repository
4. External clients can retrieve responses from the repository

## Quick Start

### Configuration

Create a `bapao.config.json` file in the project root:

```json
{
  "access_token": "your_gitee_access_token",
  "user_name": "your_gitee_username", 
  "repo": "your_repository_name",
  "file_path": "communication_file_name"
}
```

### Running the Application

```bash
# Build the project
cargo build --release

# Run the application
cargo run
```

## Features

- **Screenshot Capture**: Capture and transmit screenshots through the communication channel
- **File Transfer**: Support for both text and binary file transmission
- **Request/Response Pattern**: Structured request-response communication
- **State Management**: Track request processing states (Pending/Done)
- **Automatic Cleanup**: Expired data cleanup (30-minute expiration)

## API Documentation

For detailed API documentation, see the following sections:

- [Application Protocol API](./app_protocol_api.md)
- [Transport Protocol API](./transport_protocol_api.md)
- [Configuration Guide](./configuration.md)
- [Examples](./examples.md)

## Security Considerations

- Store Gitee access tokens securely
- Use private repositories for sensitive communications
- Regularly rotate access tokens
- Monitor repository access logs

## Contributing

This system is designed for internal-external network communication scenarios where traditional network connections are not available or restricted.
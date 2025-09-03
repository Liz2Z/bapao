# Configuration Guide

## Overview

Bapao requires a configuration file to connect to your Gitee repository. This guide explains how to set up and configure the system.

## Configuration File

### Location

The configuration file must be named `bapao.config.json` and placed in the project root directory (same directory as `Cargo.toml`).

### Format

```json
{
  "access_token": "your_gitee_personal_access_token",
  "user_name": "your_gitee_username",
  "repo": "your_repository_name",
  "file_path": "communication_file_name"
}
```

### Configuration Fields

#### `access_token` (required)

Your Gitee Personal Access Token with repository read/write permissions.

**How to obtain:**
1. Log in to Gitee
2. Go to Settings → Personal Access Tokens
3. Create a new token with the following permissions:
   - `projects` (repository access)
   - `api` (API access)

**Example:**
```json
{
  "access_token": "4d1a774f17472e4caa236205cb6155ae"
}
```

#### `user_name` (required)

Your Gitee username (the account that owns the repository).

**Example:**
```json
{
  "user_name": "myusername"
}
```

#### `repo` (required)

The name of the Gitee repository to use as a communication channel.

**Requirements:**
- Repository must exist
- Must have read/write access with the provided access token
- Recommended to use a private repository for security

**Example:**
```json
{
  "repo": "bapao-communication"
}
```

#### `file_path` (required)

The name of the file within the repository to use for communication.

**Requirements:**
- File must exist in the repository root
- Should contain valid JSON (empty array `[]` initially)
- Recommended to use a descriptive name like `io`, `messages`, or `communication`

**Example:**
```json
{
  "file_path": "io"
}
```

## Complete Configuration Example

```json
{
  "access_token": "4d1a774f17472e4caa236205cb6155ae",
  "user_name": "johndoe",
  "repo": "secure-communication",
  "file_path": "messages"
}
```

## Repository Setup

### 1. Create Repository

1. Create a new repository on Gitee (preferably private)
2. Note the repository name for the configuration

### 2. Initialize Communication File

Create the communication file in your repository:

1. In your repository, create a new file with the name specified in `file_path`
2. Initialize it with an empty JSON array: `[]`
3. Commit the file

**Example file content:**
```json
[]
```

### 3. Verify Permissions

Ensure your access token has the necessary permissions:

```bash
# Test API access (replace with your values)
curl -H "Authorization: token YOUR_ACCESS_TOKEN" \
  "https://gitee.com/api/v5/repos/USERNAME/REPO/contents/FILE_PATH"
```

## Security Best Practices

### Access Token Security

- **Never commit access tokens to version control**
- Store tokens in environment variables or secure configuration management
- Regularly rotate access tokens
- Use minimal required permissions

### Repository Security

- **Use private repositories** for sensitive communications
- Regularly audit repository access
- Monitor repository activity logs
- Consider using organization repositories with team access controls

### Configuration File Security

```bash
# Add to .gitignore to prevent committing sensitive config
echo "bapao.config.json" >> .gitignore

# Set restrictive file permissions
chmod 600 bapao.config.json
```

## Environment-Specific Configuration

### Development Configuration

```json
{
  "access_token": "dev_token_here",
  "user_name": "dev-user",
  "repo": "bapao-dev",
  "file_path": "dev-messages"
}
```

### Production Configuration

```json
{
  "access_token": "prod_token_here", 
  "user_name": "prod-user",
  "repo": "bapao-prod",
  "file_path": "messages"
}
```

## Configuration Validation

The system will validate your configuration on startup. Common errors:

### Invalid Access Token
```
Error: HTTP 401 Unauthorized
Solution: Check your access token and permissions
```

### Repository Not Found
```
Error: HTTP 404 Not Found
Solution: Verify repository name and access permissions
```

### File Not Found
```
Error: HTTP 404 Not Found (file)
Solution: Create the communication file in your repository
```

### Invalid JSON in Communication File
```
Error: JSON parsing error
Solution: Ensure the communication file contains valid JSON (start with [])
```

## Configuration Loading

The configuration is loaded automatically when the transport protocol starts:

```rust
// Configuration is loaded internally by the transport layer
let (content, sha) = get_content().await?;  // Reads bapao.config.json automatically
```

## Troubleshooting

### Common Issues

1. **"Config file not found"**
   - Ensure `bapao.config.json` is in the project root
   - Check file permissions

2. **"Invalid JSON in config"**
   - Validate JSON syntax
   - Ensure all required fields are present

3. **"Authentication failed"**
   - Verify access token is correct
   - Check token permissions include repository access

4. **"Repository access denied"**
   - Ensure the user has access to the repository
   - Verify repository name is correct

### Debug Mode

Enable debug logging to troubleshoot configuration issues:

```bash
RUST_LOG=debug cargo run
```

## Migration

When updating configuration:

1. Stop the running application
2. Update `bapao.config.json`
3. Restart the application
4. Verify connection with new settings

The system does not support hot-reloading of configuration changes.
# Nyx - Asymmetric Encryption Service

A Rust-based web service that provides secure content encryption and decryption using asymmetric (RSA) encryption.
Each piece of content gets its own unique key pair, ensuring maximum security.

## Features

- **Two REST endpoints**:
  - `POST /encrypt` - Encrypts content and returns a unique key
  - `GET /decrypt/:key` - Decrypts content using the key (content is deleted after decryption)
- **Asymmetric encryption** using RSA 2048-bit keys
- **PostgreSQL** database for secure storage
- **One-time access** - content is automatically deleted after decryption
- **Unique key pairs** - each encrypted content gets its own RSA key pair

## Setup

### Prerequisites

- Rust (latest stable)
- Docker and Docker Compose
- PostgreSQL (or use the provided Docker setup)

### Quick Start

1. **Clone and navigate to the project**:

   ```bash
   cd Nyx
   ```

2. **Start PostgreSQL database**:

   ```bash
   docker-compose up -d
   ```

3. **Run the server**:
   ```bash
   cargo run
   ```

The server will start on `http://localhost:3000`

## Configuration

Nyx uses a layered configuration system with the following precedence (highest to lowest):

1. **Environment variables** (highest priority)
2. **config.toml file**
3. **Built-in defaults** (lowest priority)

### Configuration Methods

#### 1. Using config.toml file

The `config.toml` file in the project root contains all configuration options:

```toml
[server]
host = "0.0.0.0"
port = 3000

[database]
url = "postgresql://nyx_user:nyx_password@localhost:5432/nyx_db"
max_connections = 10

[encryption]
key_size = 2048
```

#### 2. Using Environment Variables

All configuration can be overridden using environment variables with the `NYX_` prefix:

```bash
# Server configuration
export NYX_SERVER__HOST="127.0.0.1"
export NYX_SERVER__PORT=8080

# Database configuration
export NYX_DATABASE__URL="postgresql://user:pass@host:5432/db"
export NYX_DATABASE__MAX_CONNECTIONS=20

# Encryption configuration
export NYX_ENCRYPTION__KEY_SIZE=4096
```

#### 3. Custom Config File

You can specify a custom config file path:

```bash
export NYX_CONFIG_PATH="/path/to/custom/config"
cargo run
```

### Configuration Options

| Section | Option | Default | Description |
|---------|--------|---------|-------------|
| `server.host` | Host address | `0.0.0.0` | Server bind address |
| `server.port` | Port number | `3000` | Server port |
| `database.url` | Database URL | `postgresql://nyx_user:nyx_password@localhost:5432/nyx_db` | PostgreSQL connection string |
| `database.max_connections` | Max connections | `10` | Maximum database connections |
| `encryption.key_size` | RSA key size | `2048` | RSA key size in bits (1024, 2048, 4096) |

## API Usage

### Encrypt Content

**POST** `/encrypt`

```bash
curl -X POST http://localhost:3000/encrypt \
  -H "Content-Type: application/json" \
  -d '{"content": "Your secret message"}'
```

Response:

```json
{
  "key": "550e8400-e29b-41d4-a716-446655440000"
}
```

### Decrypt Content

**GET** `/decrypt/:key`

```bash
curl http://localhost:3000/decrypt/550e8400-e29b-41d4-a716-446655440000
```

Response:

```json
{
  "content": "Your secret message"
}
```

**Note**: After successful decryption, the content is permanently deleted from the database.

## Security Features

- **RSA 2048-bit encryption** for strong security
- **Unique key pairs** for each encrypted content
- **Automatic content deletion** after decryption
- **Base64 encoded** encrypted data for safe transport
- **No private key exposure** - keys are stored securely in the database

## Database Schema

```sql
CREATE TABLE encrypted_content (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    encrypted_data TEXT NOT NULL,
    private_key TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## Development

### Build

```bash
cargo build
```

### Run tests

```bash
cargo test
```

### Format code

```bash
cargo fmt
```

### Check code

```bash
cargo clippy
```

## Architecture

The service uses:

- **Axum** for the web framework
- **SQLx** for database operations
- **RSA crate** for asymmetric encryption
- **UUID** for unique key generation
- **Base64** for safe data encoding

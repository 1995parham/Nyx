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

3. **Set up environment**:

   ```bash
   cp .env.example .env
   ```

4. **Run the server**:
   ```bash
   cargo run
   ```

The server will start on `http://localhost:3000`

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

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string (default: `postgresql://nyx_user:nyx_password@localhost:5432/nyx_db`)
- `PORT` - Server port (default: `3000`)

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

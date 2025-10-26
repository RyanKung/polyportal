# PolyPortal CLI

A Rust-based command line tool for deploying and interacting with the PolyPortal smart contract.

## Features

- **Secure key management**: Private keys are encrypted and password-protected
- **Easy key import**: Interactive key import with password encryption
- Deploy PolyPortal contract
- Add/remove endpoints
- Add/remove admins
- Query endpoints (list, count, check existence)
- Configurable network support via `config.toml`

## Security

- Private keys are encrypted using AES-256-GCM
- Password-based encryption with Argon2 key derivation
- Passwords are never stored in plain text
- Interactive password prompts for secure operations

## Installation

```bash
cd rust
cargo build --release
```

The binary will be located at `target/release/polyportal-cli`.

## First Time Setup

### 1. Import Your Private Key

First, import and encrypt your private key:

```bash
cargo run -- import-key
```

This will:
- Prompt you for your private key
- Ask you to set a password
- Encrypt your private key
- Save the encrypted key and your address to `config.toml`

### 2. Edit Network Configuration

Edit `config.toml` to configure your target network:

```toml
[network]
name = "localhost"  # Network name
rpc_url = "http://127.0.0.1:8545"  # RPC endpoint
chain_id = 1337  # Chain ID
```

The deployer section will be automatically populated when you import your key.

## Usage

### Import Your Private Key

Before deploying, you need to import and encrypt your private key:

```bash
cargo run -- import-key
```

### Deploy Contract

Deploy the contract (you'll be prompted for your password):

```bash
cargo run -- deploy
```

### Add Endpoint

```bash
cargo run -- add-endpoint \
  --contract 0x1234... \
  --url https://api.example.com
```

### Remove Endpoint

```bash
cargo run -- remove-endpoint \
  --contract 0x1234... \
  --url https://api.example.com
```

### Add Admin

```bash
cargo run -- add-admin \
  --contract 0x1234... \
  --admin 0x5678...
```

### Remove Admin

```bash
cargo run -- remove-admin \
  --contract 0x1234... \
  --admin 0x5678...
```

### Get All Endpoints

```bash
cargo run -- get-endpoints --contract 0x1234...
```

### Get Endpoint Count

```bash
cargo run -- get-count --contract 0x1234...
```

### Check Endpoint Existence

```bash
cargo run -- has-endpoint \
  --contract 0x1234... \
  --url https://api.example.com
```

## Network Examples

### Localhost (Hardhat)

```toml
[network]
name = "localhost"
rpc_url = "http://127.0.0.1:8545"
chain_id = 1337
```

### Sepolia Testnet

```toml
[network]
name = "sepolia"
rpc_url = "https://sepolia.infura.io/v3/YOUR_INFURA_KEY"
chain_id = 11155111
```

### Mainnet

```toml
[network]
name = "mainnet"
rpc_url = "https://mainnet.infura.io/v3/YOUR_INFURA_KEY"
chain_id = 1
```

## Security Notes

- Private keys are encrypted with your password using AES-256-GCM
- Passwords are never stored - you'll be prompted for each operation
- Never share your password or encrypted key
- Keep your `config.toml` file secure (it's git-ignored by default)
- Use strong passwords (minimum 8 characters recommended)

## Commands

| Command | Description |
|---------|-------------|
| `import-key` | Import and encrypt your private key |
| `deploy` | Deploy the PolyPortal contract |
| `add-endpoint` | Add a new endpoint |
| `remove-endpoint` | Remove an endpoint |
| `add-admin` | Add a new admin |
| `remove-admin` | Remove an admin |
| `get-endpoints` | List all endpoints |
| `get-count` | Get endpoint count |
| `has-endpoint` | Check if endpoint exists |

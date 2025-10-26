# PolyPortal

A decentralized service registration and discovery contract for blockchain networks.

## Overview

PolyPortal is a smart contract that enables service registration and discovery on the blockchain. It provides a decentralized way for services to register their endpoints and for clients to discover available services.

## Features

### Admin Management (Owner Only)
- **Add Admin**: Add a new administrator who can manage endpoints
- **Remove Admin**: Remove an administrator's privileges

### Endpoint Management (Admin/Owner Only)
- **Add Endpoint**: Register a new service endpoint
- **Remove Endpoint**: Remove a service endpoint from the registry

### View Functions (Public)
- **Get Endpoint Count**: Get the total number of registered endpoints
- **Get All Endpoints**: Retrieve all registered endpoints as an array
- **Get Endpoint by Index**: Get a specific endpoint by its index
- **Check Endpoint Existence**: Verify if an endpoint is registered

### Ownership Management
- **Transfer Ownership**: Transfer contract ownership to a new address

## Architecture

```
Owner
  ├─ Manages Admins (add/remove)
  ├─ Manages Endpoints (add/remove)
  └─ Can transfer ownership
      
Admins
  └─ Manage Endpoints (add/remove)
      
Public
  └─ View endpoints (read-only)
```

### Permission Model

- **Owner**: Has full control over the contract
  - Can add/remove admins
  - Can add/remove endpoints
  - Can transfer ownership

- **Admins**: Can manage endpoints
  - Can add/remove endpoints
  - Cannot manage other admins
  - Cannot transfer ownership

- **Public**: Read-only access
  - Can view all endpoints
  - Can check endpoint existence

## Project Structure

```
polyportal/
├── contracts/
│   └── PolyEndpoint.sol          # Main smart contract
├── test/
│   └── PolyEndpoint.test.js       # Hardhat test suite
├── test-forge/
│   └── PolyEndpoint.t.sol         # Foundry test suite
├── scripts/
│   └── deploy.js                  # Deployment script
├── rust/
│   ├── src/
│   │   ├── main.rs                # CLI tool
│   │   ├── config.rs           # Config management
│   │   └── crypto.rs             # Encryption
│   └── config.toml               # CLI config (git-ignored)
├── hardhat.config.js             # Hardhat config
├── foundry.toml                  # Foundry config
└── README.md                      # This file
```

## Getting Started

### Prerequisites

- Node.js (v16 or higher)
- npm or yarn

### Installation

```bash
npm install
```

### Development

Compile contracts:
```bash
npm run compile
```

Run Hardhat tests:
```bash
npm run test
```

Run Foundry/Anvil tests:
```bash
npm run test:forge
```

Start local Anvil node:
```bash
npm run test:anvil
```

Deploy contracts:
```bash
npm run deploy
```

## Usage Examples

### As Owner

```javascript
// Add an admin
await polyEndpoint.addAdmin(adminAddress);

// Remove an admin
await polyEndpoint.removeAdmin(adminAddress);

// Add an endpoint
await polyEndpoint.addEndpoint("https://api.example.com");

// Remove an endpoint
await polyEndpoint.removeEndpoint("https://api.example.com");

// Transfer ownership
await polyEndpoint.transferOwnership(newOwnerAddress);
```

### As Admin

```javascript
// Add an endpoint
await polyEndpoint.connect(admin).addEndpoint("https://service.example.com");

// Remove an endpoint
await polyEndpoint.connect(admin).removeEndpoint("https://service.example.com");
```

### As Public User (Read-Only)

```javascript
// Get all endpoints
const endpoints = await polyEndpoint.getAllEndpoints();

// Check if endpoint exists
const exists = await polyEndpoint.hasEndpoint("https://api.example.com");

// Get endpoint by index
const endpoint = await polyEndpoint.getEndpoint(0);

// Get endpoint count
const count = await polyEndpoint.getEndpointCount();
```

## Events

The contract emits the following events:

- `AdminAdded(address indexed admin)`: Emitted when an admin is added
- `AdminRemoved(address indexed admin)`: Emitted when an admin is removed
- `EndpointAdded(string indexed endpoint)`: Emitted when an endpoint is added
- `EndpointRemoved(string indexed endpoint)`: Emitted when an endpoint is removed
- `OwnershipTransferred(address indexed previousOwner, address indexed newOwner)`: Emitted when ownership is transferred

## Security Considerations

- Only the owner can add/remove admins
- Only admins and owner can add/remove endpoints
- Empty endpoints are rejected
- Duplicate endpoints are prevented
- Zero address is not allowed for admins or ownership

## Testing

The test suite includes comprehensive tests for:
- Admin management (add/remove)
- Endpoint management (add/remove)
- Permission checks
- View functions
- Edge cases and error handling

Run tests with:
```bash
npm run test
```

## CLI Tool (Rust)

A secure command-line tool is provided for deploying and interacting with the PolyPortal contract. See [rust/README.md](rust/README.md) for details.

### Key Features

- **Secure key management**: Private keys are encrypted with password protection
- **Interactive key import**: `cargo run -- import-key`
- **Password-protected operations**: Deploy and interact with password prompts

### Quick Start

```bash
cd rust

# 1. Import your private key
cargo run -- import-key

# 2. Deploy (will prompt for password)
cargo run -- deploy
```

The CLI encrypts your private key and stores it securely in `config.toml` (git-ignored by default).

## License

MIT
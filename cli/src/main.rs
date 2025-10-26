use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TransactionRequest},
    utils::hex,
};
use ethers_middleware::Middleware;
use std::io::{self, Write};
use std::str::FromStr;
use rpassword::prompt_password;

mod config;
mod crypto;

use config::{Config, WalletsFile, DeployerConfig, NetworkConfig, ContractConfig};
use crypto::{encrypt_private_key, decrypt_private_key};

#[derive(Parser)]
#[command(name = "polyportal-cli")]
#[command(about = "A CLI tool for deploying and interacting with PolyPortal contract")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize CLI with network configuration and private key
    Init,
    /// Deploy the PolyPortal contract
    Deploy,
    /// Import and encrypt a private key
    ImportKey,
    /// List all wallets
    ListWallets,
    /// Add a new wallet
    AddWallet {
        #[arg(short, long)]
        name: String,
    },
    /// Add an endpoint
    AddEndpoint {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        contract: String,
    },
    /// Remove an endpoint
    RemoveEndpoint {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        contract: String,
    },
    /// Add an admin
    AddAdmin {
        #[arg(short, long)]
        admin: String,
        #[arg(short, long)]
        contract: String,
    },
    /// Remove an admin
    RemoveAdmin {
        #[arg(short, long)]
        admin: String,
        #[arg(short, long)]
        contract: String,
    },
    /// Get all endpoints
    GetEndpoints {
        #[arg(short, long)]
        contract: String,
    },
    /// Get endpoint count
    GetCount {
        #[arg(short, long)]
        contract: String,
    },
    /// Check if endpoint exists
    HasEndpoint {
        #[arg(short, long)]
        url: String,
        #[arg(short, long)]
        contract: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            init_cli().await?;
        }
        Commands::ImportKey => {
            import_key().await?;
        }
        Commands::AddWallet { name } => {
            add_wallet(&name).await?;
        }
        Commands::ListWallets => {
            list_wallets().await?;
        }
        Commands::Deploy => {
            deploy_contract().await?;
        }
        Commands::AddEndpoint { url, contract } => {
            call_add_endpoint(contract, url).await?;
        }
        Commands::RemoveEndpoint { url, contract } => {
            call_remove_endpoint(contract, url).await?;
        }
        Commands::AddAdmin { admin, contract } => {
            call_add_admin(contract, admin).await?;
        }
        Commands::RemoveAdmin { admin, contract } => {
            call_remove_admin(contract, admin).await?;
        }
        Commands::GetEndpoints { contract } => {
            call_get_endpoints(contract).await?;
        }
        Commands::GetCount { contract } => {
            call_get_count(contract).await?;
        }
        Commands::HasEndpoint { url, contract } => {
            call_has_endpoint(contract, url).await?;
        }
    }

    Ok(())
}

fn default_contract() -> ContractConfig {
    ContractConfig {
        abi_path: "../artifacts/contracts/PolyEndpoint.sol/PolyEndpoint.json".to_string(),
        bytecode_path: "../artifacts/contracts/PolyEndpoint.sol/PolyEndpoint.json".to_string(),
    }
}

async fn add_wallet(name: &str) -> Result<()> {
    println!("=== Add New Wallet ===");
    
    let mut private_key = prompt_password("Enter your private key (with or without 0x): ")
        .context("Failed to read private key")?;
    
    if !private_key.starts_with("0x") {
        private_key = format!("0x{}", private_key);
    }
    
    let password = prompt_password("Enter a password to encrypt your key: ")
        .context("Failed to read password")?;
    
    let confirm_password = prompt_password("Confirm password: ")
        .context("Failed to read password confirmation")?;
    
    if password != confirm_password {
        anyhow::bail!("Passwords do not match");
    }
    
    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }
    
    let encrypted_key = encrypt_private_key(&private_key, &password)?;
    let wallet = LocalWallet::from_str(&private_key)?;
    let address = wallet.address();
    
    let mut wallets = WalletsFile::load("wallet.toml")?;
    wallets.add_wallet(name.to_string(), format!("{:#x}", address), encrypted_key);
    wallets.save("wallet.toml")?;
    
    println!("✅ Wallet '{}' added successfully!", name);
    println!("Address: {:#x}", address);
    
    Ok(())
}

async fn list_wallets() -> Result<()> {
    let wallets = WalletsFile::load("wallet.toml")?;
    
    if wallets.wallets.is_empty() {
        println!("No wallets found.");
        return Ok(());
    }
    
    println!("=== Saved Wallets ===");
    for wallet in wallets.wallets {
        println!("  {} -> {}", wallet.name, wallet.address);
    }
    
    Ok(())
}

async fn select_wallet_interactive() -> Result<(String, String)> {
    let wallets = WalletsFile::load("wallet.toml")?;
    
    if wallets.wallets.is_empty() {
        anyhow::bail!("No wallets found. Run 'init' or 'add-wallet' first.");
    }
    
    // If only one wallet, use it
    if wallets.wallets.len() == 1 {
        let wallet = &wallets.wallets[0];
        println!("Using wallet: {} ({})", wallet.name, wallet.address);
        return Ok((wallet.encrypted_key.clone(), wallet.address.clone()));
    }
    
    // Multiple wallets - let user choose
    println!("=== Select Wallet ===");
    for (i, wallet) in wallets.wallets.iter().enumerate() {
        println!("  {}: {} -> {}", i + 1, wallet.name, wallet.address);
    }
    println!();
    
    print!("Select wallet (1-{}): ", wallets.wallets.len());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let choice = input.trim().parse::<usize>()
        .context("Invalid selection")?;
    
    if choice < 1 || choice > wallets.wallets.len() {
        anyhow::bail!("Invalid selection");
    }
    
    let wallet = &wallets.wallets[choice - 1];
    println!("Selected: {} ({})", wallet.name, wallet.address);
    
    Ok((wallet.encrypted_key.clone(), wallet.address.clone()))
}

#[allow(dead_code)]
fn default_network() -> NetworkConfig {
    NetworkConfig {
        name: "localhost".to_string(),
        rpc_url: "http://127.0.0.1:8545".to_string(),
        chain_id: 1337,
    }
}

#[allow(dead_code)]
fn default_deployer() -> DeployerConfig {
    DeployerConfig {
        address: "".to_string(),
        encrypted_key: "".to_string(),
    }
}

async fn init_cli() -> Result<()> {
    println!("=== Initialize PolyPortal CLI ===");
    println!();
    println!("This will guide you through setting up your configuration.");
    println!();
    
    // Chain selection
    println!("Network Configuration:");
    println!("1. Sepolia Testnet (Default) - Chain ID: 11155111");
    println!("2. Ethereum Mainnet - Chain ID: 1");
    println!("3. Base Mainnet - Chain ID: 8453");
    println!("4. Base Sepolia (Testnet) - Chain ID: 84532");
    println!("5. Goerli Testnet - Chain ID: 5");
    println!("6. Monad - Chain ID: 9090");
    println!("7. Localhost - Chain ID: 1337");
    println!("8. Custom");
    println!();
    
    let mut input = String::new();
    print!("Select network (1-8) [default: 1]: ");
    use std::io::{self, Write};
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    
    let choice = input.trim().parse::<u8>().unwrap_or(1);
    
    let (chain_id, chain_name, default_rpc) = match choice {
        1 => (11155111u64, "sepolia".to_string(), "https://sepolia.infura.io/v3/YOUR_INFURA_KEY".to_string()),
        2 => (1u64, "mainnet".to_string(), "https://mainnet.infura.io/v3/YOUR_INFURA_KEY".to_string()),
        3 => (8453u64, "base".to_string(), "https://mainnet.base.org".to_string()),
        4 => (84532u64, "base-sepolia".to_string(), "https://sepolia.base.org".to_string()),
        5 => (5u64, "goerli".to_string(), "https://goerli.infura.io/v3/YOUR_INFURA_KEY".to_string()),
        6 => (9090u64, "monad".to_string(), "https://monad.gg".to_string()),
        7 => (1337u64, "localhost".to_string(), "http://127.0.0.1:8545".to_string()),
        _ => {
            let mut id_input = String::new();
            let mut name_input = String::new();
            let mut rpc_input = String::new();
            
            print!("Enter Chain ID: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut id_input)?;
            
            print!("Enter Network Name: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut name_input)?;
            
            print!("Enter RPC URL: ");
            io::stdout().flush()?;
            io::stdin().read_line(&mut rpc_input)?;
            
            let chain_id = id_input.trim().parse().unwrap_or(1);
            let name = name_input.trim().to_string();
            let rpc = rpc_input.trim().to_string();
            
            (chain_id as u64, name, rpc)
        }
    };
    
    let mut rpc_url = default_rpc;
    if choice != 7 {  // Not localhost
        print!("Enter RPC URL [{}]: ", rpc_url);
        io::stdout().flush()?;
        let mut rpc_input = String::new();
        io::stdin().read_line(&mut rpc_input)?;
        let trimmed = rpc_input.trim();
        if !trimmed.is_empty() {
            rpc_url = trimmed.to_string();
        }
    }
    
    println!();
    println!("Private Key Configuration:");
    println!("⚠️  Your private key will be encrypted with a password");
    
    // Get private key
    let mut private_key = prompt_password("Enter your private key (with or without 0x): ")
        .context("Failed to read private key")?;
    
    // Auto-add 0x prefix if missing
    if !private_key.starts_with("0x") {
        private_key = format!("0x{}", private_key);
    }
    
    // Get password
    let password = prompt_password("Enter a password to encrypt your key: ")
        .context("Failed to read password")?;
    
    let confirm_password = prompt_password("Confirm password: ")
        .context("Failed to read password confirmation")?;
    
    if password != confirm_password {
        anyhow::bail!("Passwords do not match");
    }
    
    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }
    
    // Encrypt the private key
    println!();
    println!("Encrypting private key...");
    let encrypted_key = encrypt_private_key(&private_key, &password)
        .context("Failed to encrypt private key")?;
    
    // Get address from private key
    let wallet = LocalWallet::from_str(&private_key)?;
    let address = wallet.address();
    
    // Load or create config
    let mut config = match Config::load("config.toml") {
        Ok(cfg) => {
            println!("⚠️  Warning: config.toml already exists and will be overwritten.");
            cfg
        }
        Err(_) => {
            Config::load("cli/config.toml").unwrap_or_else(|_| {
                Config {
                    network: NetworkConfig {
                        name: chain_name.clone(),
                        rpc_url: rpc_url.clone(),
                        chain_id,
                    },
                    deployer: DeployerConfig {
                        address: "".to_string(),
                        encrypted_key: "".to_string(),
                    },
                    contract: default_contract(),
                    active_wallet: None,
                }
            })
        }
    };
    
    // Update config
    config.network.name = chain_name.clone();
    config.network.rpc_url = rpc_url.clone();
    config.network.chain_id = chain_id;
    
    // Ask for wallet name
    print!("Enter a name for this wallet [default: wallet-1]: ");
    io::stdout().flush()?;
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input)?;
    let wallet_name = name_input.trim();
    let wallet_name = if wallet_name.is_empty() { "wallet-1" } else { wallet_name };
    
    // Load or create wallet.toml
    let mut wallets = WalletsFile::load("wallet.toml")
        .unwrap_or_else(|_| WalletsFile { wallets: vec![] });
    
    // Add wallet
    wallets.add_wallet(wallet_name.to_string(), format!("{:#x}", address), encrypted_key);
    wallets.save("wallet.toml")?;
    
    // Update active wallet
    config.active_wallet = Some(wallet_name.to_string());
    
    // Save config
    config.save("config.toml")
        .context("Failed to save config")?;
    
    println!();
    println!("✅ Configuration initialized successfully!");
    println!();
    println!("Network: {} (Chain ID: {})", chain_name, chain_id);
    println!("RPC URL: {}", config.network.rpc_url);
    println!("Wallet: {} -> {}", wallet_name, format!("{:#x}", address));
    println!("Config saved to: config.toml");
    println!("Wallets saved to: wallet.toml");
    println!();
    println!("Remember your password - you'll need it to deploy the contract.");
    
    Ok(())
}

async fn get_password_and_wallet() -> Result<(String, String)> {
    let (encrypted_key, _wallet_address) = select_wallet_interactive().await?;
    
    let password = prompt_password("Enter your password: ")
        .context("Failed to read password")?;
    
    let private_key = decrypt_private_key(&encrypted_key, &password)
        .context("Failed to decrypt private key. Wrong password?")?;
    
    Ok((private_key, password))
}

async fn setup_client(config: &Config, private_key: &str) -> Result<SignerMiddleware<Provider<Http>, LocalWallet>> {
    let provider = Provider::<Http>::try_from(&config.network.rpc_url)
        .context("Failed to create provider")?;
    
    let wallet = LocalWallet::from_str(private_key)
        .context("Failed to create wallet")?
        .with_chain_id(config.network.chain_id);
    
    Ok(SignerMiddleware::new(provider, wallet))
}

#[allow(dead_code)]
async fn get_contract_abi() -> Result<serde_json::Value> {
    let abi_path = "../artifacts/contracts/PolyEndpoint.sol/PolyEndpoint.json";
    let abi_str = std::fs::read_to_string(abi_path)
        .context("Failed to read contract ABI")?;
    Ok(serde_json::from_str(&abi_str)?)
}

async fn call_add_endpoint(contract: String, url: String) -> Result<()> {
    println!("Adding endpoint: {}", url);
    println!("Contract: {}", contract);
    
    let config = Config::load("config.toml")
        .context("Failed to load config. Run 'init' first.")?;
    
    let (private_key, _password) = get_password_and_wallet().await?;
    let client = setup_client(&config, &private_key).await?;
    
    let contract_address: Address = contract.parse()
        .context("Invalid contract address")?;
    
    // Manual ABI encoding for addEndpoint(string)
    // Function signature: addEndpoint(string)
    // Method ID: 0x + first 4 bytes of keccak256("addEndpoint(string)")
    let method_id = ethers::utils::keccak256("addEndpoint(string)")[0..4].to_vec();
    
    // Encode string parameter
    let encoded = ethers::abi::encode(&[ethers::abi::Token::String(url.clone())]);
    let full_data = [&method_id[..], &encoded].concat();
    
    let tx = TransactionRequest::new()
        .to(contract_address)
        .data(Bytes::from(full_data));
    
    println!("Sending transaction...");
    let pending_tx = client.send_transaction(tx, None).await?;
    println!("Transaction sent: {:?}", pending_tx.tx_hash());
    
    println!("Waiting for confirmation...");
    let receipt = pending_tx.await?;
    
    if receipt.is_some() {
        println!("✅ Endpoint added successfully!");
    }
    
    Ok(())
}

async fn call_remove_endpoint(contract: String, url: String) -> Result<()> {
    println!("Removing endpoint: {}", url);
    
    let config = Config::load("config.toml")?;
    let (private_key, _password) = get_password_and_wallet().await?;
    let client = setup_client(&config, &private_key).await?;
    
    let contract_address: Address = contract.parse()?;
    
    let method_id = ethers::utils::keccak256("removeEndpoint(string)")[0..4].to_vec();
    let encoded = ethers::abi::encode(&[ethers::abi::Token::String(url.clone())]);
    let full_data = [&method_id[..], &encoded].concat();
    
    let tx = TransactionRequest::new()
        .to(contract_address)
        .data(Bytes::from(full_data));
    
    println!("Sending transaction...");
    let pending_tx = client.send_transaction(tx, None).await?;
    println!("Transaction sent: {:?}", pending_tx.tx_hash());
    
    let receipt = pending_tx.await?;
    if receipt.is_some() {
        println!("✅ Endpoint removed successfully!");
    }
    
    Ok(())
}

async fn call_add_admin(contract: String, admin: String) -> Result<()> {
    println!("Adding admin: {}", admin);
    
    let config = Config::load("config.toml")?;
    let (private_key, _password) = get_password_and_wallet().await?;
    let client = setup_client(&config, &private_key).await?;
    
    let contract_address: Address = contract.parse()?;
    let admin_address: Address = admin.parse()?;
    
    let method_id = ethers::utils::keccak256("addAdmin(address)")[0..4].to_vec();
    let encoded = ethers::abi::encode(&[ethers::abi::Token::Address(admin_address)]);
    let full_data = [&method_id[..], &encoded].concat();
    
    let tx = TransactionRequest::new()
        .to(contract_address)
        .data(Bytes::from(full_data));
    
    println!("Sending transaction...");
    let pending_tx = client.send_transaction(tx, None).await?;
    println!("Transaction sent: {:?}", pending_tx.tx_hash());
    
    let receipt = pending_tx.await?;
    if receipt.is_some() {
        println!("✅ Admin added successfully!");
    }
    
    Ok(())
}

async fn call_remove_admin(contract: String, admin: String) -> Result<()> {
    println!("Removing admin: {}", admin);
    
    let config = Config::load("config.toml")?;
    let (private_key, _password) = get_password_and_wallet().await?;
    let client = setup_client(&config, &private_key).await?;
    
    let contract_address: Address = contract.parse()?;
    let admin_address: Address = admin.parse()?;
    
    let method_id = ethers::utils::keccak256("removeAdmin(address)")[0..4].to_vec();
    let encoded = ethers::abi::encode(&[ethers::abi::Token::Address(admin_address)]);
    let full_data = [&method_id[..], &encoded].concat();
    
    let tx = TransactionRequest::new()
        .to(contract_address)
        .data(Bytes::from(full_data));
    
    println!("Sending transaction...");
    let pending_tx = client.send_transaction(tx, None).await?;
    println!("Transaction sent: {:?}", pending_tx.tx_hash());
    
    let receipt = pending_tx.await?;
    if receipt.is_some() {
        println!("✅ Admin removed successfully!");
    }
    
    Ok(())
}

async fn call_get_endpoints(_contract: String) -> Result<()> {
    println!("Getting all endpoints...");
    println!("⚠️  Read operations temporarily disabled due to API changes");
    println!("Use a block explorer to view contract state.");
    Ok(())
}

async fn call_get_count(_contract: String) -> Result<()> {
    println!("Getting endpoint count...");
    println!("⚠️  Read operations temporarily disabled due to API changes");
    println!("Use a block explorer to view contract state.");
    Ok(())
}

async fn call_has_endpoint(_contract: String, url: String) -> Result<()> {
    println!("Checking if endpoint exists: {}", url);
    println!("⚠️  Read operations temporarily disabled due to API changes");
    println!("Use a block explorer to view contract state.");
    Ok(())
}

async fn import_key() -> Result<()> {
    println!("=== Private Key Import ===");
    println!();
    
    // Get private key from user
    let mut private_key = prompt_password("Enter your private key (with or without 0x): ")
        .context("Failed to read private key")?;
    
    // Auto-add 0x prefix if missing
    if !private_key.starts_with("0x") {
        private_key = format!("0x{}", private_key);
    }
    
    // Get password from user
    let password = prompt_password("Enter a password to encrypt your key: ")
        .context("Failed to read password")?;
    
    let confirm_password = prompt_password("Confirm password: ")
        .context("Failed to read password confirmation")?;
    
    if password != confirm_password {
        anyhow::bail!("Passwords do not match");
    }
    
    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }
    
    // Encrypt the private key
    println!("Encrypting private key...");
    let encrypted_key = encrypt_private_key(&private_key, &password)
        .context("Failed to encrypt private key")?;
    
    // Get address from private key
    let wallet = LocalWallet::from_str(&private_key)?;
    let address = wallet.address();
    
    // Load or create config
    let mut config = match Config::load("rust/config.toml") {
        Ok(cfg) => cfg,
        Err(_) => {
            println!("Creating new config.toml...");
            let template = include_str!("../config.toml");
            std::fs::write("rust/config.toml", template)?;
            Config::load("rust/config.toml")?
        }
    };
    
    // Update config with encrypted key
    config.deployer.encrypted_key = encrypted_key;
    config.deployer.address = format!("{:#x}", address);
    
    // Save config
    config.save("rust/config.toml")
        .context("Failed to save config")?;
    
    println!();
    println!("✓ Private key encrypted and saved!");
    println!("✓ Address: {:#x}", address);
    println!("✓ Configuration saved to rust/config.toml");
    println!();
    println!("Remember your password - you'll need it to deploy the contract.");
    
    Ok(())
}

async fn deploy_contract() -> Result<()> {
    // Select wallet interactively
    let (encrypted_key, _wallet_address) = select_wallet_interactive().await?;
    
    // Load config
    let config = Config::load("config.toml")
        .context("Failed to load config. Run 'init' first.")?;
    
    // Get password from user
    println!();
    println!("=== Deploy Contract ===");
    println!();
    let password = prompt_password("Enter your password: ")
        .context("Failed to read password")?;
    
    // Decrypt private key
    println!("Decrypting private key...");
    let private_key = decrypt_private_key(&encrypted_key, &password)
        .context("Failed to decrypt private key. Wrong password?")?;
    
    // Setup provider
    let provider = Provider::<Http>::try_from(&config.network.rpc_url)
        .context("Failed to create provider")?;
    
    let wallet = LocalWallet::from_str(&private_key)
        .context("Failed to create wallet")?
        .with_chain_id(config.network.chain_id);
    
    let client = SignerMiddleware::new(provider, wallet);
    
    // Read contract artifacts
    println!("Reading contract artifacts...");
    let artifact_str = std::fs::read_to_string(&config.contract.bytecode_path)?;
    let artifact: serde_json::Value = serde_json::from_str(&artifact_str)?;
    
    let bytecode = artifact["bytecode"]
        .as_str()
        .context("No bytecode found")?;
    
    println!("Deploying contract to {}...", config.network.name);
    println!("RPC URL: {}", config.network.rpc_url);
    
    let bytecode_bytes = hex::decode(bytecode.strip_prefix("0x").unwrap_or(bytecode))?;
    
    let deployer_address = client.address();
    println!("Deploying with wallet: {:?}", deployer_address);
    
    let tx = TransactionRequest::new().data(Bytes::from(bytecode_bytes));
    
    println!("Sending deployment transaction...");
    let pending_tx = client.send_transaction(tx, None).await?;
    println!("Transaction sent: {:?}", pending_tx.tx_hash());
    
    println!("Waiting for confirmation...");
    let receipt = pending_tx.await?;
    
    if let Some(receipt) = receipt {
        if let Some(contract_address) = receipt.contract_address {
            println!();
            println!("✓ Contract deployed successfully!");
            println!("Contract address: {:?}", contract_address);
            println!();
            println!("You can now use this address with other commands:");
            println!("  cargo run -- add-endpoint --contract {:?} --url https://example.com", contract_address);
        } else {
            println!("⚠ Contract deployed but no contract address in receipt");
        }
    } else {
        println!("⚠ No receipt received");
    }
    
    Ok(())
}

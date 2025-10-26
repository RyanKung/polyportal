use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "default_network")]
    pub network: NetworkConfig,
    #[serde(default = "default_deployer")]
    pub deployer: DeployerConfig,
    #[serde(default = "default_contract")]
    pub contract: ContractConfig,
    #[serde(default)]
    pub active_wallet: Option<String>,
}

fn default_deployer() -> DeployerConfig {
    DeployerConfig {
        address: "".to_string(),
        encrypted_key: "".to_string(),
    }
}

fn default_network() -> NetworkConfig {
    NetworkConfig {
        name: "localhost".to_string(),
        rpc_url: "http://127.0.0.1:8545".to_string(),
        chain_id: 1337,
    }
}

fn default_contract() -> ContractConfig {
    ContractConfig {
        abi_path: "../artifacts/contracts/PolyEndpoint.sol/PolyEndpoint.json".to_string(),
        bytecode_path: "../artifacts/contracts/PolyEndpoint.sol/PolyEndpoint.json".to_string(),
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NetworkConfig {
    pub name: String,
    pub rpc_url: String,
    pub chain_id: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeployerConfig {
    pub address: String,
    pub encrypted_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WalletEntry {
    pub name: String,
    pub address: String,
    pub encrypted_key: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WalletsFile {
    pub wallets: Vec<WalletEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContractConfig {
    pub abi_path: String,
    pub bytecode_path: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let config_str = fs::read_to_string(path)
            .context("Failed to read config.toml")?;
        toml::from_str(&config_str)
            .context("Failed to parse config.toml")
    }

    pub fn save(&self, path: &str) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = std::path::Path::new(path).parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }
        
        let toml = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        fs::write(path, toml)
            .context("Failed to write config.toml")
    }
}

impl WalletsFile {
    pub fn load(wallet_path: &str) -> Result<Self> {
        if !Path::new(wallet_path).exists() {
            return Ok(WalletsFile { wallets: vec![] });
        }
        let wallet_str = fs::read_to_string(wallet_path)
            .context("Failed to read wallet.toml")?;
        toml::from_str(&wallet_str)
            .context("Failed to parse wallet.toml")
    }

    pub fn save(&self, wallet_path: &str) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = std::path::Path::new(wallet_path).parent() {
            fs::create_dir_all(parent)
                .context("Failed to create wallet directory")?;
        }
        
        let toml = toml::to_string_pretty(self)
            .context("Failed to serialize wallet config")?;
        fs::write(wallet_path, toml)
            .context("Failed to write wallet.toml")
    }

    pub fn add_wallet(&mut self, name: String, address: String, encrypted_key: String) {
        let wallet = WalletEntry {
            name,
            address,
            encrypted_key,
        };
        self.wallets.push(wallet);
    }

    #[allow(dead_code)]
    pub fn get_wallet(&self, name: &str) -> Option<&WalletEntry> {
        self.wallets.iter().find(|w| w.name == name)
    }

}
//! PolyEndpoint SDK
//! A WASM-compatible SDK for interacting with the PolyEndpoint contract

// Re-export main modules
pub mod abi;
pub mod contract;

// Client module (only available with ethers feature)
#[cfg(feature = "ethers")]
pub mod client;

// WASM-specific exports
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(feature = "ethers")]
pub use client::PolyEndpointClient;


//! PolyEndpoint SDK
//! Simple SDK that fetches endpoint lists from PolyEndpoint smart contract

mod simple_client;
mod endpoint;
mod http_impl;

pub use simple_client::PolyEndpointClient;
pub use endpoint::EndpointInfo;


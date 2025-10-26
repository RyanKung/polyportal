//! Endpoint information types

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EndpointInfo {
    pub url: String,
    pub description: String,
}

impl EndpointInfo {
    pub fn new(url: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            description: description.into(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}


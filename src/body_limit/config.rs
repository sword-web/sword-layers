use crate::{DisplayConfig, utils::ByteConfig};
use byte_unit::Byte;
use console::style;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// ### Body Limit Configuration
///
/// Configuration for the Body Limit Layer
/// This configuration allows you to set a maximum size for request bodies.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BodyLimitConfig {
    /// The maximum allowed size for request bodies (e.g., "1MB", "500KB").
    #[serde(rename = "max-size")]
    pub max_size: ByteConfig,
    /// Whether to display the configuration details.
    pub display: bool,
}

impl DisplayConfig for BodyLimitConfig {
    fn display(&self) {
        if self.display {
            println!("\n{}", style("Body Limit Configuration:").bold());
            println!("  ↳  Max Body Size: {}", self.max_size.raw);
        }
    }
}

impl Default for BodyLimitConfig {
    fn default() -> Self {
        let max_size = "10MB".to_string();
        let parsed = Byte::from_str(&max_size)
            .unwrap_or_else(|_| Byte::from_u64(10 * 1024 * 1024))
            .as_u64() as usize;

        BodyLimitConfig {
            display: true,
            max_size: ByteConfig {
                parsed,
                raw: max_size,
            },
        }
    }
}

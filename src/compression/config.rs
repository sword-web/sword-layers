use crate::DisplayConfig;
use console::style;
use serde::{Deserialize, Serialize};

/// ### Compression Configuration
///
/// Configuration for the Compression Layer
/// This configuration allows you to enable or disable compression
/// and specify which algorithms to use.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct CompressionConfig {
    /// A boolean indicating if compression is enabled.
    pub enabled: bool,
    /// A list of strings representing the compression algorithms to use
    /// (e.g., "gzip", "deflate", "br", "zstd
    pub algorithms: Vec<String>,
    /// Whether to display the configuration details.
    pub display: bool,
}

impl DisplayConfig for CompressionConfig {
    fn display(&self) {
        if !self.display {
            return;
        }

        println!("\n{}", style("Compression Configuration:").bold());

        if self.enabled {
            if self.algorithms.is_empty() {
                println!("  ↳  {}", style("No algorithms enabled").yellow());
            } else {
                println!("  ↳  Algorithms: {}", self.algorithms.join(", "));
            }
        } else {
            println!("  ↳  {}", style("Compression: disabled").red());
        }
    }
}

impl Default for CompressionConfig {
    fn default() -> Self {
        CompressionConfig {
            enabled: false,
            algorithms: Vec::new(),
            display: false,
        }
    }
}

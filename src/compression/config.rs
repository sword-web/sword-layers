use console::style;
use serde::{Deserialize, Serialize};

use crate::DisplayConfig;

/// ### Compression Configuration
/// Configuration for the Compression Layer
/// This configuration allows you to enable or disable compression
/// and specify which algorithms to use.
///
/// #### Fields:
/// - `enabled`: A boolean indicating if compression is enabled.
/// - `algorithms`: A list of strings representing the compression algorithms to use
/// (e.g., "gzip", "deflate", "br", "zstd")
///
/// - `display`: A boolean indicating if the configuration should be displayed.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CompressionConfig {
    #[serde(default)]
    pub enabled: bool,

    #[serde(default)]
    pub algorithms: Vec<String>,

    #[serde(default)]
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

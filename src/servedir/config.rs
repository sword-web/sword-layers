use console::style;
use serde::{Deserialize, Serialize};

use crate::DisplayConfig;

/// ### Serve Directory Configuration
/// Configuration for the Serve Directory Layer
/// This configuration allows you to set up static file serving with optional compression.
///
/// #### Fields:
/// - `enabled`: A boolean indicating if static file serving is enabled.
/// - `static_dir`: The directory path containing static files to serve.
/// - `router_path`: The route path where static files will be accessible.
/// - `compression_algorithm`: Optional compression algorithm ("gzip", "br", "deflate", "zstd").
/// - `chunk_size`: Optional chunk size for streaming files (e.g., "64KB").
/// - `not_found_file`: Optional custom 404 file path.
/// - `display`: A boolean indicating if the configuration should be displayed.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct ServeDirConfig {
    pub enabled: bool,
    pub static_dir: String,
    pub router_path: String,
    pub compression_algorithm: Option<String>,
    pub chunk_size: Option<String>,
    pub not_found_file: Option<String>,

    #[serde(default)]
    pub display: bool,
}

impl DisplayConfig for ServeDirConfig {
    fn display(&self) {
        if !self.display {
            return;
        }

        println!("\n{}", style("Serve Directory Configuration:").bold());
        println!("  ↳  Enabled: {}", self.enabled);
        println!("  ↳  Static Directory: {}", self.static_dir);
        println!("  ↳  Router Path: {}", self.router_path);

        if let Some(algorithm) = &self.compression_algorithm {
            println!("  ↳  Compression Algorithm: {algorithm}");
        }
        if let Some(chunk_size) = &self.chunk_size {
            println!("  ↳  Chunk Size: {chunk_size}");
        }
        if let Some(not_found) = &self.not_found_file {
            println!("  ↳  Not Found File: {not_found}");
        }
    }
}

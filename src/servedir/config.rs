use crate::{DisplayConfig, utils::ByteConfig};
use console::style;
use serde::{Deserialize, Serialize};

/// ### Serve Directory Configuration
///
/// Configuration for the Serve Directory Layer
/// This configuration allows you to set up static file serving using the ServeDir `tower` layer.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct ServeDirConfig {
    /// Boolean indicator whether the Serve Directory Layer is enabled. Defaults to false.
    pub enabled: bool,

    /// The directory path containing static files to serve. Defaults to "public".
    #[serde(rename = "static-dir")]
    pub static_dir: String,

    /// The route path where static files will be accessible. Defaults to "/static".
    #[serde(rename = "router-path")]
    pub router_path: String,

    /// The compression algorithm to use for serving files. (e.g., "gzip", "br"). Defaults to None.
    #[serde(rename = "compression-algorithm")]
    pub compression_algorithm: Option<String>,

    /// The chunk size for streaming files (e.g., "64KB"). Defaults to None.
    #[serde(rename = "chunk-size")]
    pub chunk_size: Option<ByteConfig>,

    /// The custom 404 file path to serve when a file is not found. Defaults to None.
    #[serde(rename = "not-found-file")]
    pub not_found_file: Option<String>,

    /// Whether to display the configuration details. Defaults to false.
    pub display: bool,
}

impl Default for ServeDirConfig {
    fn default() -> Self {
        ServeDirConfig {
            enabled: false,
            static_dir: "public".to_string(),
            router_path: "/static".to_string(),
            compression_algorithm: None,
            chunk_size: None,
            not_found_file: None,
            display: false,
        }
    }
}

impl DisplayConfig for ServeDirConfig {
    fn display(&self) {
        if !self.display {
            return;
        }

        println!("\n{}", style("Serve Directory Configuration:").bold());
        println!("  ↳  Enabled: {}", self.enabled);

        println!(
            "  ↳  Paths: static: {} - router: {}",
            self.static_dir, self.router_path
        );

        let mut options = Vec::new();

        if let Some(algorithm) = &self.compression_algorithm {
            options.push(format!("compression: {}", algorithm));
        }
        if let Some(chunk_size) = &self.chunk_size {
            options.push(format!("chunk size: {}", chunk_size.raw));
        }
        if let Some(not_found) = &self.not_found_file {
            options.push(format!("404 file: {}", not_found));
        }
        if !options.is_empty() {
            println!("  ↳  Options: {}", options.join(" - "));
        }
    }
}

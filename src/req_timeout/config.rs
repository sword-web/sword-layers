use crate::{DisplayConfig, utils::TimeConfig};
use console::style;
use duration_str::parse as parse_duration;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// ### Request Timeout Configuration
///
/// Configuration for the Request Timeout Layer
/// This configuration allows you to set a maximum duration for request handling.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RequestTimeoutConfig {
    /// Boolean indicating if request timeout is enabled. Defaults to false.
    pub enabled: bool,
    /// The timeout duration as a string (e.g., "30s", "1m"). Defaults to "15s".
    pub timeout: TimeConfig,
    /// Whether to display the configuration details. Defaults to false.
    pub display: bool,
}

impl DisplayConfig for RequestTimeoutConfig {
    fn display(&self) {
        if !self.display {
            return;
        }

        println!("\n{}", style("Request Timeout Configuration:").bold());

        if self.enabled {
            println!("  ↳  Request Timeout: {}", self.timeout.raw);
        } else {
            println!("  ↳  Request Timeout: disabled");
        }
    }
}

impl Default for RequestTimeoutConfig {
    fn default() -> Self {
        let duration_str = "15s".to_string();
        let parsed_duration =
            parse_duration(&duration_str).unwrap_or_else(|_| Duration::from_secs(15));

        RequestTimeoutConfig {
            enabled: false,
            timeout: TimeConfig {
                parsed: parsed_duration,
                raw: duration_str,
            },
            display: false,
        }
    }
}

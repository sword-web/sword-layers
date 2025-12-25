use crate::DisplayConfig;
use console::style;
use serde::{Deserialize, Serialize};

/// ### CORS Configuration
/// Configuration for the CORS Layer
/// This configuration allows you to control cross-origin resource sharing policies.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct CorsConfig {
    /// Boolean indicating if CORS is enabled.
    pub enabled: bool,

    /// A list of allowed origins for cross-origin requests.
    #[serde(rename = "allow-origins")]
    pub allow_origins: Option<Vec<String>>,

    /// A list of allowed HTTP methods (e.g., "GET", "POST").
    #[serde(rename = "allow-methods")]
    pub allow_methods: Option<Vec<String>>,

    /// A list of allowed HTTP headers.
    #[serde(rename = "allow-headers")]
    pub allow_headers: Option<Vec<String>>,

    /// A boolean indicating if credentials are allowed in cross-origin requests.
    #[serde(rename = "allow-credentials")]
    pub allow_credentials: Option<bool>,

    /// The maximum age in seconds for CORS preflight responses.
    #[serde(rename = "max-age")]
    pub max_age: Option<u64>,

    /// Whether to display the configuration details.
    pub display: bool,
}

impl DisplayConfig for CorsConfig {
    fn display(&self) {
        if !self.display {
            return;
        }

        println!("\n{}", style("CORS Configuration:").bold());

        println!("  ↳  Enabled: {}", self.enabled);

        if let Some(origins) = &self.allow_origins {
            println!("  ↳  Allowed Origins: {origins:?}");
        }

        if let Some(methods) = &self.allow_methods {
            println!("  ↳  Allowed Methods: {methods:?}");
        }

        if let Some(headers) = &self.allow_headers {
            println!("  ↳  Allowed Headers: {headers:?}");
        }

        if let Some(credentials) = self.allow_credentials {
            println!("  ↳  Allow Credentials: {credentials}");
        }

        if let Some(max_age) = self.max_age {
            println!("  ↳  Max Age: {max_age}s");
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        CorsConfig {
            enabled: false,
            allow_origins: None,
            allow_methods: None,
            allow_headers: None,
            allow_credentials: None,
            max_age: None,
            display: false,
        }
    }
}

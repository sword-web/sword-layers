use console::style;
use serde::{Deserialize, Serialize};

use crate::DisplayConfig;

/// ### CORS Configuration
/// Configuration for the CORS Layer
/// This configuration allows you to control cross-origin resource sharing policies.
///
/// #### Fields:
/// - `enabled`: A boolean indicating if CORS is enabled.
/// - `allow_origins`: A list of allowed origins for cross-origin requests.
/// - `allow_methods`: A list of allowed HTTP methods (e.g., "GET", "POST").
/// - `allow_headers`: A list of allowed HTTP headers.
/// - `allow_credentials`: A boolean indicating if credentials are allowed in cross-origin requests.
/// - `max_age`: The maximum age in seconds for CORS preflight responses.
/// - `display`: A boolean indicating if the configuration should be displayed.
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct CorsConfig {
    pub enabled: bool,
    pub allow_origins: Option<Vec<String>>,
    pub allow_methods: Option<Vec<String>>,
    pub allow_headers: Option<Vec<String>>,
    pub allow_credentials: Option<bool>,
    pub max_age: Option<u64>,

    #[serde(default)]
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

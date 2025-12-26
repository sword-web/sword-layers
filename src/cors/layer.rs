use super::CorsConfig;

use axum::http::{HeaderName, HeaderValue, Method};
use tower_http::cors::CorsLayer as TowerCorsLayer;

/// ### CORS Layer
///
/// This struct represents the CORS Layer which
/// handles Cross-Origin Resource Sharing configuration.
///
/// The layer is a wrapper around `tower_http::cors::CorsLayer`.
/// Configures which origins, methods, headers, and credentials are allowed
/// for cross-origin requests.
pub struct CorsLayer;

impl CorsLayer {
    pub fn new(config: &CorsConfig) -> TowerCorsLayer {
        let mut layer = TowerCorsLayer::new();

        if let Some(allow_credentials) = config.allow_credentials {
            layer = layer.allow_credentials(allow_credentials);
        }

        if let Some(origin) = &config.allow_origins {
            let parsed_origin: Vec<HeaderValue> = origin
                .iter()
                .filter_map(|o| HeaderValue::from_str(o).ok())
                .collect();

            layer = layer.allow_origin(parsed_origin);
        }

        if let Some(methods) = &config.allow_methods {
            let parsed_methods: Vec<Method> =
                methods.iter().filter_map(|m| m.parse().ok()).collect();

            layer = layer.allow_methods(parsed_methods);
        }

        if let Some(headers) = &config.allow_headers {
            let parsed_headers: Vec<HeaderName> =
                headers.iter().filter_map(|h| h.parse().ok()).collect();

            layer = layer.allow_headers(parsed_headers);
        }

        if let Some(max_age) = &config.max_age {
            layer = layer.max_age(max_age.parsed);
        }

        layer
    }
}

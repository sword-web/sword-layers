use super::CompressionConfig;
use tower_http::compression::CompressionLayer as TowerCompressionLayer;

/// ### Compression Layer
///
/// This struct represents the Compression Layer which
/// enables response compression based on the provided configuration.
///
/// The layer is a wrapper around `tower_http::compression::CompressionLayer`.
/// Compress response bodies of the underlying service.
///
/// This uses the Accept-Encoding header to pick an appropriate encoding
/// and adds the Content-Encoding header to responses.
pub struct CompressionLayer;

impl CompressionLayer {
    pub fn new(config: CompressionConfig) -> TowerCompressionLayer {
        let mut layer = TowerCompressionLayer::new();

        for algorithm in &config.algorithms {
            match algorithm.to_lowercase().as_str() {
                "gzip" => layer = layer.gzip(true),
                "deflate" => layer = layer.deflate(true),
                "br" | "brotli" => layer = layer.br(true),
                "zstd" => layer = layer.zstd(true),
                _ => {}
            }
        }

        layer
    }
}

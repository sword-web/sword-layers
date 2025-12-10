use super::ServeDirConfig;

use byte_unit::Byte;
use tower_http::services::{ServeDir, ServeFile};

/// ### Serve Directory Layer
///
/// This struct represents the Serve Directory Layer which
/// serves static files and directories.
///
/// The layer is a wrapper around `tower_http::services::ServeDir`.
/// Serves static content with optional compression and custom chunk sizing.
pub struct ServeDirLayer;

impl ServeDirLayer {
    pub fn new(config: &ServeDirConfig) -> ServeDir<ServeFile> {
        let mut fallback = ServeFile::new(format!("{}/404.html", config.static_dir));

        if let Some(not_found_file) = &config.not_found_file {
            fallback = ServeFile::new(format!("{}/{not_found_file}", config.static_dir));
        }

        let mut layer = ServeDir::new(&config.static_dir).fallback(fallback);

        if let Some(algorithm) = &config.compression_algorithm {
            match algorithm.as_str() {
                "br" => {
                    layer = layer.precompressed_br();
                }
                "gzip" => {
                    layer = layer.precompressed_gzip();
                }
                "deflate" => {
                    layer = layer.precompressed_deflate();
                }
                "zstd" => {
                    layer = layer.precompressed_zstd();
                }
                _ => {}
            }
        }

        if let Some(chunk_size_str) = &config.chunk_size
            && let Ok(chunk_size) = chunk_size_str.parse::<Byte>()
        {
            layer = layer.with_buf_chunk_size(chunk_size.as_u64() as usize);
        }

        layer
    }
}

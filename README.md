# Tower Layers for Sword Web Framework

> <img src="https://avatars.githubusercontent.com/u/228345998?s=200&v=4" align="right" width="120"/>

This crate provides a collection of **Tower middleware layers** with built-in configuration support for the [Sword Web Framework](https://github.com/sword-web/sword). Each layer is designed to handle common HTTP concerns like compression, CORS, rate limiting, and static file serving, while maintaining a consistent configuration pattern across the framework.

## Available Layers

| Layer               | Description                                   | Purpose                                                                            |
| ------------------- | --------------------------------------------- | ---------------------------------------------------------------------------------- |
| **Body Limit**      | Restricts the size of incoming request bodies | Prevents oversized uploads and protects against denial-of-service attacks          |
| **Compression**     | Enables response compression                  | Reduces response size using gzip, deflate, brotli, or zstd algorithms              |
| **CORS**            | Cross-Origin Resource Sharing configuration   | Controls which origins, methods, and headers are allowed for cross-origin requests |
| **Request Timeout** | Enforces maximum request duration             | Prevents hung connections and timeouts incomplete requests                         |
| **Serve Directory** | Serves static files and directories           | Provides efficient static content delivery with optional compression               |

## Configuration Pattern

Each layer follows a consistent configuration pattern:

```rust
// Each layer has:
// 1. A configuration struct (e.g., BodyLimitConfig)
// 2. A layer struct with a `new()` method that accepts the config (e.g., BodyLimitLayer)
// 3. Standardized error responses using sword-responses

pub struct LayerConfig {
    pub enabled: bool,
    // ... other fields
    pub display: bool,
}

pub struct Layer;

impl Layer {
    pub fn new(config: LayerConfig) -> /* Layer Type */ {
        // Implementation
    }
}
```

## How to Add Tower Layers

Each layer consists of two main components: a configuration struct and a layer struct that wraps the Tower middleware. If you want to contribute a new layer, follow these steps:

1. **Create a configuration struct** that holds all the settings for your layer. This struct should implement the `DisplayConfig` trait to allow users to see what settings are active. Include an `enabled` field to allow toggling the layer on and off, and a `display` field to control whether configuration details are printed.

2. **Implement a layer struct** that acts as a factory for creating the actual Tower layer. This struct typically has a single `new()` method that accepts the configuration and returns the configured Tower layer instance. The layer struct is the public API that users interact with.

3. **Handle custom responses** when necessary. If your layer needs to return custom error responses, use the `ResponseFnMapper` to standardize them according to the sword-responses format.

4. **Export everything** from the library. Add your new module to `src/lib.rs` and export both the configuration struct and the layer struct so they're available to users.

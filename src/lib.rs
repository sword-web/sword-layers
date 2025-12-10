mod content_type;
pub mod helmet;

use axum::{body::Body, http::Response};
pub use content_type::ContentTypeCheck;

pub(crate) type ResponseFnMapper = fn(Response<Body>) -> Response<Body>;

pub mod compression {
    mod config;
    mod layer;

    pub use config::CompressionConfig;
    pub use layer::CompressionLayer;
}

pub mod cors {
    mod config;
    mod layer;

    pub use config::CorsConfig;
    pub use layer::CorsLayer;
}

pub mod servedir {
    mod config;
    mod layer;

    pub use config::ServeDirConfig;
    pub use layer::ServeDirLayer;
}

pub mod body_limit {
    mod config;
    mod layer;

    pub use config::BodyLimitConfig;
    pub use layer::BodyLimitLayer;
}

pub mod req_timeout {
    mod config;
    mod layer;

    pub use config::RequestTimeoutConfig;
    pub use layer::RequestTimeoutLayer;
}

pub trait DisplayConfig {
    fn display(&self);
}

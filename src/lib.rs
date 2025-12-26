pub mod helmet;
pub mod not_found;
pub mod prelude;
pub mod utils;

use axum::{body::Body, http::Response};

pub(crate) type ResponseFnMapper = fn(Response<Body>) -> Response<Body>;

pub mod socketio {
    mod config;
    mod layer;

    pub use config::*;
    pub use layer::*;
}

pub mod cookies {
    pub use tower_cookies::*;
}

pub mod request_id {
    mod layer;

    pub use layer::{RequestId, RequestIdLayer};
    pub use uuid::Uuid;
}

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

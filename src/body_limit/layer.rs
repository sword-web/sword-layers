use crate::{ResponseFnMapper, body_limit::BodyLimitConfig};

use axum_responses::JsonResponse;
use tower::{
    ServiceBuilder,
    layer::util::{Identity, Stack},
    util::MapResponseLayer,
};

use tower_http::limit::RequestBodyLimitLayer;

use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

type BodyLimitLayerType = ServiceBuilder<
    Stack<MapResponseLayer<ResponseFnMapper>, Stack<RequestBodyLimitLayer, Identity>>,
>;

/// ### Body Limit Layer
///
/// This structs represents the Body Limit Layer which
/// restricts the size of incoming request bodies.
///
/// The layer is in fact a `ServiceBuilder` that applies a body size limit
/// and maps responses to `sword` standardized responses.
pub struct BodyLimitLayer;

impl BodyLimitLayer {
    pub fn new(config: &BodyLimitConfig) -> BodyLimitLayerType {
        fn map_body_limit_response(r: Response<Body>) -> Response<Body> {
            if r.status().as_u16() != 413 {
                return r;
            }

            JsonResponse::PayloadTooLarge()
                .message("The request body exceeds the maximum allowed size by the server")
                .into_response()
        }

        ServiceBuilder::new()
            .layer(RequestBodyLimitLayer::new(config.parsed))
            .map_response(map_body_limit_response as ResponseFnMapper)
    }
}

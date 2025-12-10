use crate::ResponseFnMapper;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use axum_responses::JsonResponse;
use std::time::Duration;
use tower::{ServiceBuilder, util::MapResponseLayer};
use tower_http::timeout::TimeoutLayer as TowerTimeoutLayer;
use tower_layer::{Identity, Stack};

type TimeoutLayerType = (
    TowerTimeoutLayer,
    ServiceBuilder<Stack<MapResponseLayer<ResponseFnMapper>, Identity>>,
);

/// ### Request Timeout Layer
///
/// This struct represents the Request Timeout Layer which
/// enforces a maximum duration for incoming requests.
///
/// The layer is a combination of `tower_http::timeout::TimeoutLayer` and a response mapper.
/// Returns a 408 Request Timeout status when a request exceeds the configured duration.
pub struct RequestTimeoutLayer;

impl RequestTimeoutLayer {
    pub fn new(duration: Duration) -> TimeoutLayerType {
        let layer = TowerTimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, duration);

        fn timeout_mapper(response: Response) -> Response {
            if response.status().as_u16() == 408 {
                return JsonResponse::RequestTimeout().into_response();
            }

            response
        }

        let response_layer = ServiceBuilder::new().map_response(timeout_mapper as ResponseFnMapper);

        (layer, response_layer)
    }
}

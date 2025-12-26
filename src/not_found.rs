use crate::ResponseFnMapper;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_responses::JsonResponse;
use tower::{ServiceBuilder, util::MapResponseLayer};
use tower_layer::{Identity, Stack};

pub struct NotFoundLayer;

impl NotFoundLayer {
    pub fn new() -> ServiceBuilder<Stack<MapResponseLayer<ResponseFnMapper>, Identity>> {
        ServiceBuilder::new().map_response(|r: Response| {
            if r.status() != StatusCode::NOT_FOUND {
                return r;
            }

            JsonResponse::NotFound()
                .message("The requested resource was not found.")
                .into_response()
        })
    }
}

use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_layer::{Identity, Stack};

type RequestIdServiceType = ServiceBuilder<
    Stack<PropagateRequestIdLayer, Stack<SetRequestIdLayer<MakeRequestUuid>, Identity>>,
>;

pub use tower_http::request_id::RequestId;

pub struct RequestIdLayer;

impl RequestIdLayer {
    pub fn new() -> RequestIdServiceType {
        ServiceBuilder::new()
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid::default()))
            .layer(PropagateRequestIdLayer::x_request_id())
    }
}

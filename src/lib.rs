pub mod helmet;
mod limits;

pub type ResponseFnMapper =
    fn(axum::http::Response<axum::body::Body>) -> axum::http::Response<axum::body::Body>;

define_layer_mod!(compression);
define_layer_mod!(servedir);
define_layer_mod!(cors);

// define_layer_mod("compression");
// generates:
// mod compression {
//    mod layer;
//    pub use layer::*;
//    mod config;
//    pub use config::*;
// }

macro_rules! define_layer_mod {
    ($layer_name:ident) => {
        mod $layer_name {
            mod layer;
            pub use layer::*;
            mod config;
            pub use config::*;
        }
    };
}

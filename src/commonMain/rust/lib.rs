#![allow(clippy::module_inception)]
pub mod env;
pub mod model {
    pub use model::*;

    mod apple_model_field;
    mod model;
}
pub mod protobuf {
    pub use stremio_core_protobuf::protobuf::*;
}
pub mod stremio_core_apple;

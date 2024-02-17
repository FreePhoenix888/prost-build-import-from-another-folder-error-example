use serde::{Deserialize, Serialize};
use prost::Message;

// Include the `settings` module, which is generated from settings.proto.
pub mod settings {
    include!(concat!(env!("OUT_DIR"), "/settings.rs"));
}

// Include the `storage_settings` module, which is generated from settings.proto.
pub mod storage_settings {
    include!(concat!(env!("OUT_DIR"), "/storage_settings.rs"));
}
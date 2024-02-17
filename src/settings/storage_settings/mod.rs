use serde::{Deserialize, Serialize};
use prost::Message;

// Include the `storage_settings` module, which is generated from storage_settings.proto.
pub mod storage_settings {
    include!(concat!(env!("OUT_DIR"), "/storage_settings.rs"));
}


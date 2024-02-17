mod storage_settings;

use serde::{Deserialize, Serialize};
use prost::Message;

// Include the `settings` module, which is generated from settings.proto.
pub mod settings {
    include!(concat!(env!("OUT_DIR"), "/settings.rs"));
}


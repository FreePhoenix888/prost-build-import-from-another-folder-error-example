extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["src/settings/settings.proto"],
                                &["src/", "src/settings", "src/settings/storage_settings"]).unwrap();
}

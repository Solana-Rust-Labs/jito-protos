use std::path::Path;
use tonic_build::configure;

fn main() {
    const PROTOC_ENVAR: &str = "PROTOC";
    unsafe {
        if std::env::var(PROTOC_ENVAR).is_err() {
            #[cfg(not(windows))]
            std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
        }
    }

    // Try to find the protos directory relative to this build.rs file
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let protos_dir = Path::new(&manifest_dir).join("protos");

    let proto_files = [
        "auth.proto",
        "block_engine.proto",
        "block.proto",
        "bundle.proto",
        "packet.proto",
        "relayer.proto",
        "searcher.proto",
        "shared.proto",
        "shredstream.proto",
    ];

    let proto_paths: Vec<String> = proto_files
        .iter()
        .map(|file| protos_dir.join(file).to_string_lossy().to_string())
        .collect();

    configure()
        .compile(&proto_paths, &[protos_dir.to_string_lossy().as_ref()])
        .unwrap();
}

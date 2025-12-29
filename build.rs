use std::io::Result;

fn main() -> Result<()> {
    // Compile all proto files in the proto directory
    let proto_files = vec![
        "proto/common.proto",
        "proto/component.proto",
        "proto/messaging.proto",
        "proto/config.proto",
    ];

    // Configure prost-build to compile the proto files
    prost_build::Config::new()
        .out_dir("src/generated")
        .compile_protos(&proto_files, &["proto/"])?;

    // Tell cargo to recompile if any proto file changes
    for file in &proto_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    Ok(())
}

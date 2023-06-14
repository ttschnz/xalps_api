extern crate protoc_rust;

fn main() {
    println!("cargo build");

    protoc_rust::Codegen::new()
        .out_dir("./src/status")
        // .protoc_path("./src/status")
        .inputs(&["./src/status/track_response.proto"])
        // .include("protos")
        .run()
        .expect("Running protoc failed.");
}

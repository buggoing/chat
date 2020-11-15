fn main() {
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/proto")
        .compile(&["proto/chat.proto"], &["proto"])
        .expect("failed to compile protos");
}




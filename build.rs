fn main() {
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/proto")
        .compile(&["proto/post.proto"], &["proto"])
        .expect("failed to compile protos");
}

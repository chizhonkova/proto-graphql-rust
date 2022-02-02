#![warn(rust_2018_idioms)]

fn main() {
    let config = proto_graphql_build::Config::new();
    proto_graphql_build::configure()
        .out_dir("src/generated")
        .remove_scalar_wrappers(true)
        .allow_nullable_lists(true)
        .compile_with_config(config, &["proto/helloworld.proto"], &["proto"])
        .unwrap();
}

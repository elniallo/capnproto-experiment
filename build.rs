extern crate capnpc;

fn main() {
     capnpc::CompilerCommand::new()
        .file("src/schema/test_schema.capnp")
        .src_prefix("src/schema")
        .edition(capnpc::RustEdition::Rust2018)
        .output_path("src/schema")
        .run()
        .expect("compiling schema");
}

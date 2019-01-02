extern crate capnpc;

fn main() {
     capnpc::CompilerCommand::new()
        .file("src/schema/test_schema.capnp")
        .src_prefix("src/schema")
        .output_path("src/schema_out")
        .run()
        .expect("compiling schema");
}

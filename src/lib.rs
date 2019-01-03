extern crate capnp;
extern crate capnp_rpc;
pub mod test_schema_capnp {
    include!(concat!("schema", "/test_schema_capnp.rs"));
}
pub mod server;

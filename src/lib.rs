extern crate capnp;
#[macro_use]
extern crate capnp_rpc;
extern crate tokio;
extern crate tokio_core;
pub mod test_schema_capnp {
    include!(concat!("schema", "/test_schema_capnp.rs"));
}
pub mod client;
pub mod server;

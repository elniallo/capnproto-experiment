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

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() >= 2 {
        match &args[1][..] {
            "client" => return client::client::main(),
            "server" => return server::server::main(),
            _ => (),
        }
    }

    println!("usage: {} [client | server] ADDRESS", args[0]);
}

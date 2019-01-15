use crate::test_schema_capnp::{network, status};
use capnp::capability::Promise;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::future::Future;
use tokio::io::AsyncRead;
use tokio_core::reactor;
pub fn main() {
    use std::net::ToSocketAddrs;
    let args: Vec<String> = ::std::env::args().collect();

    if args.len() != 3 {
        println!("usage: {} server HOST:PORT", args[0]);
        return;
    }
    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");

    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    let stream = runtime
        .block_on(::tokio_core::net::TcpStream::connect(&addr, &handle))
        .unwrap();
    println!("Connected to {}", &addr);
    stream.set_nodelay(true).expect("Some error");
    let (reader, writer) = stream.split();
    let network_obj = Box::new(twoparty::VatNetwork::new(
        reader,
        writer,
        rpc_twoparty_capnp::Side::Client,
        Default::default(),
    ));
    let mut rpc_system = RpcSystem::new(network_obj, None);
    let net: network::Client = rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
    let request = net.get_status_request();
    let guid = request.send().pipeline.get_status().guid_request().send();
    let (_res, data) = runtime.block_on(rpc_system.join(guid.promise)).unwrap();
    println!("GUID: {}", data.get().unwrap().get_guid().unwrap());
    println!("message sent");
    println!("Exiting");
}

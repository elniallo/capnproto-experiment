use crate::test_schema_capnp::network;
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
        .block_on(::tokio::net::TcpStream::connect(&addr))
        .unwrap();
    println!("Connected to {}", &stream.local_addr().unwrap());
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

    let mut request = net.get_status_request();

    let _status = core.run(rpc_system.join(request.send().promise.and_then(|response| {
        let status = response.get().unwrap().get_status();
        Promise::ok(())
    })));
    println!("message sent");
    println!("Exiting");
}

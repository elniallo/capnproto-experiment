use crate::test_schema_capnp::network;
use capnp::capability::Promise;
use capnp::message::Builder;
use capnp::Error;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{Future, Stream};
use std::borrow::BorrowMut;
use std::borrow::ToOwned;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use tokio::runtime::current_thread;
use tokio_core::reactor;
use tokio_io::AsyncRead;
pub struct StatusImpl {
    version: i32,
    guid: String,
    public_port: i32,
    network_id: String,
    port: i32,
    builder: ::capnp_rpc::ImbuedMessageBuilder<::capnp::message::HeapAllocator>,
}

struct NetworkClient {
    client: network::Client,
}

struct BuilderWrapper<'a> {
    builder: &'a capnp::message::Builder<capnp::message::HeapAllocator>,
}

impl<'a> BuilderWrapper<'a> {
    fn new(builder: &'a capnp::message::Builder<capnp::message::HeapAllocator>) -> BuilderWrapper {
        BuilderWrapper { builder: builder }
    }
}

struct NetworkMap {
    map: HashMap<String, NetworkClient>,
}

impl NetworkMap {
    fn new() -> NetworkMap {
        NetworkMap {
            map: HashMap::new(),
        }
    }
}

impl StatusImpl {
    fn new(version: i32, public_port: i32, network_id: String, port: i32) -> StatusImpl {
        StatusImpl {
            version,
            public_port,
            network_id,
            port,
            guid: StatusImpl::generate_guid(),
            builder: ::capnp_rpc::ImbuedMessageBuilder::new(::capnp::message::HeapAllocator::new()),
        }
    }

    fn generate_guid() -> String {
        String::from("afhwjgbfdjobnqfjdfqojgadnv")
    }
}

pub struct RPCServer {
    status: StatusImpl,
}

impl RPCServer {
    fn new() -> RPCServer {
        RPCServer {
            status: StatusImpl::new(11, 8148, String::from("hycon"), 8148),
        }
    }
}

impl crate::test_schema_capnp::network::Server for RPCServer {
    fn get_status(
        &mut self,
        _params: network::GetStatusParams,
        mut results: network::GetStatusResults,
    ) -> Promise<(), ::capnp::Error> {
        println!("Received Status Call");
        let mut builder = self
            .status
            .builder
            .get_root::<network::status::Builder>()
            .unwrap();
        builder.set_guid(&self.status.guid);
        builder.set_version(self.status.version);
        builder.set_networkid(&self.status.network_id);
        builder.set_port(self.status.port);
        builder.set_public_port(self.status.public_port);
        let status_reader = builder.into_reader().to_owned();
        results.get().set_status(status_reader);
        Promise::ok(())
    }
}

pub fn main() {
    use std::net::ToSocketAddrs;
    let args: Vec<String> = ::std::env::args().collect();
    let mut status_reader: network::status::Reader;
    let mut builder = Builder::new_default();
    let mut server: RPCServer;
    if args.len() != 3 {
        println!("usage: {} server HOST:PORT", args[0]);
        return;
    }

    let mut core = reactor::Core::new().unwrap();
    let handle = core.handle();
    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");
    let socket = ::tokio_core::net::TcpListener::bind(&addr, &handle).unwrap();
    server = RPCServer::new();
    {
        let connection = network::ToClient::new(server).into_client::<capnp_rpc::Server>();
        let handle1 = handle.clone();
        let done = socket.incoming().for_each(move |(socket, addr)| {
            println!("New connection from {}", &addr.to_string());
            socket.set_nodelay(true)?;
            let (reader, writer) = socket.split();
            let handle = handle1.clone();

            let net = twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Server,
                Default::default(),
            );
            let conn = connection.clone().client;
            let rpc_system = RpcSystem::new(Box::new(net), Some(conn));
            handle.spawn(rpc_system.map_err(|_| ()));
            Ok(())
        });

        core.run(done).unwrap();
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_should_do_stuff() {
        let mut builder = Builder::new_default();
        let server = RPCServer::new();
        assert_eq!(
            server.status.guid,
            String::from("afhwjgbfdjobnqfjdfqojgadnv")
        );
    }
}

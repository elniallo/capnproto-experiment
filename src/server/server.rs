use crate::test_schema_capnp::{network, status};
use capnp::capability::Promise;
use capnp::message::Builder;

pub struct StatusImpl {
    version: i32,
    guid: String,
    public_port: i32,
    network_id: String,
    port: i32,
}

impl StatusImpl {
    fn new(version: i32, public_port: i32, network_id: String, port: i32) -> StatusImpl {
        StatusImpl {
            version,
            public_port,
            network_id,
            port,
            guid: RPCServer::generate_guid(),
        }
    }
}
pub struct RPCServer {
    status: StatusImpl,
}

impl RPCServer {
    fn new() -> RPCServer {
        RPCServer {
            status: StatusImpl::new(1, 8148, String::from("hycon"), 8148),
        }
    }

    fn generate_guid() -> String {
        String::from("afhwjgbfdjobnqfjdfqojgadnv")
    }
}

impl network::Server for RPCServer {
    fn get_status(
        &mut self,
        params: network::GetStatusParams,
        mut results: network::GetStatusResults,
    ) -> Promise<(), ::capnp::Error> {
        println!("Received Status Call");
        Promise::ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_should_do_stuff() {
        let server = RPCServer::new();
        assert_eq!(
            server.status.guid,
            String::from("afhwjgbfdjobnqfjdfqojgadnv")
        );
    }
}

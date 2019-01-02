use crate::test_schema_capnp::status;

pub struct RPCServer {
    guid: String,
}

impl RPCServer {
    fn new() -> RPCServer {
        RPCServer {
            guid: RPCServer::generate_guid(),
        }
    }

    fn generate_guid() -> String {
        String::from("afhwjgbfdjobnqfjdfqojgadnv")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_should_do_stuff() {
        let server = RPCServer::new();
        assert_eq!(server.guid, String::from("afhwjgbfdjobnqfjdfqojgadnv"));
    }
}

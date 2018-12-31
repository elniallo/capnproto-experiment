use crate::schema::test_schema_capnp::point;

pub struct RPCServer {
    port: u16,
}

impl RPCServer {
    fn new(x: i32, y: i32, ip: u16) -> RPCServer {
        let mut builder = capnp::message::Builder::new_default();
        let mut pt_msg = builder.init_root::<point::Builder>();
        pt_msg.set_x(x as i32);
        pt_msg.set_y(y as i32);
        let mut buf = Vec::new();
        capnp::serialize::write_message(&mut buf, &builder).unwrap();
        let deserialized = capnp::serialize::read_message(
            &mut buf.as_slice(),
            capnp::message::ReaderOptions::new(),
        )
        .unwrap();

        let point_reader = deserialized.get_root::<point::Reader>().unwrap();
        assert_eq!(point_reader.get_x(), x);
        assert_eq!(point_reader.get_y(), y);
        RPCServer { port: ip }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn it_should_do_stuff() {
        let _server = RPCServer::new(1, 2, 3);
    }
}

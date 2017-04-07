use std::io;
use tokio_io::codec::{Encoder, Decoder};
use bytes::BytesMut;

enum NetPacket {

}

struct NetCodec;

impl Encoder for NetCodec {
    type Item = NetPacket;
    type Error = io::Error;

    fn encode(&mut self, item: NetPacket, dst: &mut BytesMut) -> Result<(), io::Error> {
        Ok(())
    }
}
/*
impl Decoder for NetCodec {
    type Item = NetPacket;
    type Error =
}
*/

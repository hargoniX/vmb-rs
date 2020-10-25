use vmb_proto::{
    builder::MessagerBuilder,
    codec::VmbCodec
};

use tokio_util::codec::{Decoder, Encoder};
use bytes::BytesMut;

#[test]
fn it_encodes_and_decodes_read() {
    let message = MessagerBuilder::new_read(Some(120), 10, true, 15);
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_read_no_timestamp() {
    let message = MessagerBuilder::new_read(None, 10, true, 15);
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_read_no_lock() {
    let message = MessagerBuilder::new_read(Some(120), 10, false, 15);
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_read_no_lock_no_timestamp() {
    let message = MessagerBuilder::new_read(None, 10, false, 15);
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

use vmb_proto::{
    builder::MessagerBuilder,
    codec::VmbCodec
};

use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};

#[test]
fn it_encodes_and_decodes_readreply() {
    let mut payload = BytesMut::with_capacity(16);
    payload.put_slice(b"0123456789123456");
    let message = MessagerBuilder::new_readreply(Some(120), 40, true, 10, payload.freeze()).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_readreply_no_timestamp() {
    let mut payload = BytesMut::with_capacity(16);
    payload.put_slice(b"0123456789123456");
    let message = MessagerBuilder::new_readreply(None, 40, true, 10, payload.freeze()).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_readreply_no_lock() {
    let mut payload = BytesMut::with_capacity(16);
    payload.put_slice(b"0123456789123456");
    let message = MessagerBuilder::new_readreply(Some(120), 40, false, 10, payload.freeze()).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_readreply_no_lock_no_timestamp() {
    let mut payload = BytesMut::with_capacity(16);
    payload.put_slice(b"0123456789123456");
    let message = MessagerBuilder::new_readreply(None, 40, false, 10, payload.freeze()).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

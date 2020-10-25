use vmb_proto::{
    builder::MessagerBuilder,
    codec::VmbCodec
};

use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};

#[test]
fn it_encodes_and_decodes_write() {
    let mut payload = BytesMut::with_capacity(16);
    payload.put_slice(b"0123456789123456");
    let message = MessagerBuilder::new_write(Some(120), 10, true, 5, payload).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_write_no_timestamp() {
    let mut payload = BytesMut::with_capacity(16);
    payload.put_slice(b"0123456789123456");
    let message = MessagerBuilder::new_write(None, 10, true, 5, payload).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_write_no_lock() {
    let mut payload = BytesMut::with_capacity(16);
    payload.put_slice(b"0123456789123456");
    let message = MessagerBuilder::new_write(Some(120), 10, false, 5, payload).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_write_no_lock_no_timestamp() {
    let mut payload = BytesMut::with_capacity(16);
    payload.put_slice(b"0123456789123456");
    let message = MessagerBuilder::new_write(None, 10, false, 5, payload).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

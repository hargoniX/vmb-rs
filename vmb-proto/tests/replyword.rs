use vmb_proto::{
    builder::MessagerBuilder,
    codec::VmbCodec
};

use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};

#[test]
fn it_encodes_and_decodes_bytereply() {
    let mut payload = BytesMut::with_capacity(1);
    payload.put_slice(b"0");
    let message = MessagerBuilder::new_bytereply(Some(120), 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_bytereply_no_timestamp() {
    let mut payload = BytesMut::with_capacity(1);
    payload.put_slice(b"0");
    let message = MessagerBuilder::new_bytereply(None, 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_bytereply_no_lock() {
    let mut payload = BytesMut::with_capacity(1);
    payload.put_slice(b"0");
    let message = MessagerBuilder::new_bytereply(Some(120), 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_bytereply_no_lock_no_timestamp() {
    let mut payload = BytesMut::with_capacity(1);
    payload.put_slice(b"0");
    let message = MessagerBuilder::new_bytereply(None, 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_wydereply() {
    let mut payload = BytesMut::with_capacity(2);
    payload.put_slice(b"01");
    let message = MessagerBuilder::new_wydereply(Some(120), 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_wydereply_no_timestamp() {
    let mut payload = BytesMut::with_capacity(2);
    payload.put_slice(b"01");
    let message = MessagerBuilder::new_wydereply(None, 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_wydereply_no_lock() {
    let mut payload = BytesMut::with_capacity(2);
    payload.put_slice(b"01");
    let message = MessagerBuilder::new_wydereply(Some(120), 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_wydereply_no_lock_no_timestamp() {
    let mut payload = BytesMut::with_capacity(2);
    payload.put_slice(b"01");
    let message = MessagerBuilder::new_wydereply(None, 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_tetrareply() {
    let mut payload = BytesMut::with_capacity(4);
    payload.put_slice(b"0123");
    let message = MessagerBuilder::new_tetrareply(Some(120), 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_tetrareply_no_timestamp() {
    let mut payload = BytesMut::with_capacity(4);
    payload.put_slice(b"0123");
    let message = MessagerBuilder::new_tetrareply(None, 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_tetrareply_no_lock() {
    let mut payload = BytesMut::with_capacity(4);
    payload.put_slice(b"0123");
    let message = MessagerBuilder::new_tetrareply(Some(120), 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_tetrareply_no_lock_no_timestamp() {
    let mut payload = BytesMut::with_capacity(4);
    payload.put_slice(b"0123");
    let message = MessagerBuilder::new_tetrareply(None, 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

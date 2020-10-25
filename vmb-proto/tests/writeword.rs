use vmb_proto::{
    builder::MessagerBuilder,
    codec::VmbCodec
};

use tokio_util::codec::{Decoder, Encoder};
use bytes::{BytesMut, BufMut};

#[test]
fn it_encodes_and_decodes_writebyte() {
    let mut payload = BytesMut::with_capacity(1);
    payload.put_slice(b"0");
    let message = MessagerBuilder::new_writebyte(Some(120), 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writebyte_no_timestamp() {
    let mut payload = BytesMut::with_capacity(1);
    payload.put_slice(b"0");
    let message = MessagerBuilder::new_writebyte(None, 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writebyte_no_lock() {
    let mut payload = BytesMut::with_capacity(1);
    payload.put_slice(b"0");
    let message = MessagerBuilder::new_writebyte(Some(120), 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writebyte_no_lock_no_timestamp() {
    let mut payload = BytesMut::with_capacity(1);
    payload.put_slice(b"0");
    let message = MessagerBuilder::new_writebyte(None, 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writewyde() {
    let mut payload = BytesMut::with_capacity(2);
    payload.put_slice(b"01");
    let message = MessagerBuilder::new_writewyde(Some(120), 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writewyde_no_timestamp() {
    let mut payload = BytesMut::with_capacity(2);
    payload.put_slice(b"01");
    let message = MessagerBuilder::new_writewyde(None, 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writewyde_no_lock() {
    let mut payload = BytesMut::with_capacity(2);
    payload.put_slice(b"01");
    let message = MessagerBuilder::new_writewyde(Some(120), 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writewyde_no_lock_no_timestamp() {
    let mut payload = BytesMut::with_capacity(2);
    payload.put_slice(b"01");
    let message = MessagerBuilder::new_writewyde(None, 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writetetra() {
    let mut payload = BytesMut::with_capacity(4);
    payload.put_slice(b"0123");
    let message = MessagerBuilder::new_writetetra(Some(120), 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writetetra_no_timestamp() {
    let mut payload = BytesMut::with_capacity(4);
    payload.put_slice(b"0123");
    let message = MessagerBuilder::new_writetetra(None, 10, payload, true, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writetetra_no_lock() {
    let mut payload = BytesMut::with_capacity(4);
    payload.put_slice(b"0123");
    let message = MessagerBuilder::new_writetetra(Some(120), 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_writetetra_no_lock_no_timestamp() {
    let mut payload = BytesMut::with_capacity(4);
    payload.put_slice(b"0123");
    let message = MessagerBuilder::new_writetetra(None, 10, payload, false, 121).unwrap();
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

use vmb_proto::{
    builder::MessagerBuilder,
    types::Route,
    codec::VmbCodec
};

use tokio_util::codec::{Decoder, Encoder};
use bytes::BytesMut;

#[test]
fn it_encodes_and_decodes_ignore() {
    let message = MessagerBuilder::new_ignore(Some(100), Some(50), Route::from(true), false, 10);
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_ignore_no_timestamp() {
    let message = MessagerBuilder::new_ignore(None, Some(50), Route::from(true), false, 10);
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_ignore_no_address() {
    let message = MessagerBuilder::new_ignore(Some(100), None, Route::from(true), false, 10);
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

#[test]
fn it_encodes_and_decodes_ignore_no_address_no_timestamp() {
    let message = MessagerBuilder::new_ignore(None, None, Route::from(true), false, 10);
    let mut codec = VmbCodec {};
    let mut buffer = BytesMut::new();
    codec.encode(message.clone(), &mut buffer).unwrap();
    let decoded_message = codec.decode(&mut buffer).unwrap().unwrap();
    assert_eq!(message, decoded_message);
}

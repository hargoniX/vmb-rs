//! Contains a tokio server and client codec for the vmb protocol.

use crate::constants::MIN_MESSAGE_SIZE;
use crate::message::{ExtendedHeader, Header, Message};
use crate::types::Octa;

use byteorder::{BigEndian, ByteOrder};
use bytes::BytesMut;
use tokio_util::codec::Decoder;

use std::io::Error;
use std::mem;

#[derive(Copy, Clone, Debug)]
struct VmbDecoder {}

impl Decoder for VmbDecoder {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < MIN_MESSAGE_SIZE as usize {
            return Ok(None);
        }

        let header = Header::from([src[0], src[1], src[2], src[3]]);

        let (_, src) = src.split_at(4);

        let (timestamp, src) = if header.r#type.time {
            if src.len() < 4 {
                return Ok(None);
            }

            let timestamp = BigEndian::read_u32(&src[0..4]);
            let (_, src) = src.split_at(4);
            (Some(timestamp), src)
        } else {
            (None, src)
        };

        let (address, src) = if header.r#type.address {
            if src.len() < 8 {
                return Ok(None);
            }

            let address = BigEndian::read_u64(&src[0..8]);
            let (_, src) = src.split_at(8);
            (Some(address), src)
        } else {
            (None, src)
        };

        let payload = if header.r#type.payload {
            let payload_size = (header.size as usize + 1) * mem::size_of::<Octa>();

            if src.len() < payload_size {
                return Ok(None);
            }

            let mut payload = BytesMut::with_capacity(payload_size);
            payload.extend_from_slice(&src[0..payload_size]);
            Some(payload)
        } else {
            None
        };

        Ok(Some(Message {
            extended_header: ExtendedHeader {
                header,
                timestamp,
                address,
            },
            payload,
        }))
    }
}

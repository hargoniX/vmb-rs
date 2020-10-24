//! Contains a tokio server and client codec for the vmb protocol.

use crate::constants::MIN_MESSAGE_SIZE;
use crate::message::{ExtendedHeader, Header, Message};
use crate::types::Octa;

use byteorder::{BigEndian, ByteOrder};
use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

use std::io::Error;
use std::mem;

#[derive(Copy, Clone, Debug)]
pub struct VmbCodec {}

impl Decoder for VmbCodec {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < MIN_MESSAGE_SIZE as usize {
            return Ok(None);
        }

        let header = Header::from([src[0], src[1], src[2], src[3]]);

        src.advance(4);

        let timestamp = if header.r#type.time {
            if src.len() < 4 {
                return Ok(None);
            }

            let timestamp = BigEndian::read_u32(&src[0..4]);
            src.advance(4);
            Some(timestamp)
        } else {
            None
        };

        let address = if header.r#type.address {
            if src.len() < 8 {
                return Ok(None);
            }

            let address = BigEndian::read_u64(&src[0..8]);
            src.advance(8);
            Some(address)
        } else {
            None
        };

        let payload = if header.r#type.payload {
            let payload_size = (header.size as usize + 1) * mem::size_of::<Octa>();

            if src.len() < payload_size {
                return Ok(None);
            }

            Some(src.split_to(payload_size))
        } else {
            None
        };

        // Reserve 4 bytes for the next header to increase performance.
        src.reserve(4);

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

impl Encoder<Message> for VmbCodec {
    type Error = Error;

    fn encode(&mut self, msg: Message, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.reserve(MIN_MESSAGE_SIZE as usize);
        let header: u64 = msg.extended_header.header.into();
        buf.put_u64(header);

        if let Some(timestamp) = msg.extended_header.timestamp {
            buf.reserve(mem::size_of::<u32>());
            buf.put_u32(timestamp);
        }

        if let Some(address) = msg.extended_header.address {
            buf.reserve(mem::size_of::<u64>());
            buf.put_u64(address);
        }

        if let Some(payload) = msg.payload {
            buf.reserve(payload.len());
            buf.put(payload);
        }

        Ok(())
    }
}

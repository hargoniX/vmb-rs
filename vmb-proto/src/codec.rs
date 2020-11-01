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

    #[tracing::instrument]
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < MIN_MESSAGE_SIZE as usize {
            // Reserve enough bytes so we get our header next time.
            src.reserve(MIN_MESSAGE_SIZE as usize - src.len());
            return Ok(None);
        }

        let header = Header::from([src[0], src[1], src[2], src[3]]);
        tracing::debug!("Decoded header: {:?}", header);

        let payload_size = if header.r#type.payload {
            // 8 * (SIZE + 1) payload
            (header.size as usize + 1) * mem::size_of::<Octa>()
        } else {
            0
        };
        tracing::debug!("Expect payload size: {:?}", payload_size);

        let remaining_length =
            // 4 byte timestamp
            4 * header.r#type.time as usize +
            // 8 byte address
            8 * header.r#type.address as usize +
            payload_size;

        if src.len() < remaining_length {
            src.reserve(remaining_length - src.len());
            return Ok(None);
        }

        src.advance(4);

        let timestamp = if header.r#type.time {
            let timestamp = BigEndian::read_u32(&src[0..4]);
            src.advance(4);
            tracing::debug!("Timestamp : {:#032b}", timestamp);
            Some(timestamp)
        } else {
            None
        };

        let address = if header.r#type.address {
            let address = BigEndian::read_u64(&src[0..8]);
            src.advance(8);
            tracing::debug!("Address : {:#064b}", address);
            Some(address)
        } else {
            None
        };

        let payload = if header.r#type.payload {
            let p = src.split_to(payload_size).freeze();
            tracing::debug!("Payload : {:?}", p);
            Some(p)
        } else {
            None
        };

        if src.len() < MIN_MESSAGE_SIZE as usize {
            // Reserve enough bytes so we get our header instantly next time.
            src.reserve(MIN_MESSAGE_SIZE as usize - src.len());
        }

        let msg = Message {
            extended_header: ExtendedHeader {
                header,
                timestamp,
                address
            },
            payload
        };

        tracing::info!("Constructed Message: {:?}", msg);

        Ok(Some(msg))
    }
}

impl Encoder<Message> for VmbCodec {
    type Error = Error;

    #[tracing::instrument]
    fn encode(&mut self, msg: Message, buf: &mut BytesMut) -> Result<(), Self::Error> {
        buf.reserve(MIN_MESSAGE_SIZE as usize);
        let header: u32 = msg.extended_header.header.into();
        tracing::debug!("Header : {:#032b}", header);
        buf.put_u32(header);

        if let Some(timestamp) = msg.extended_header.timestamp {
            buf.reserve(mem::size_of::<u32>());
            tracing::debug!("Timestamp : {:#032b}", timestamp);
            buf.put_u32(timestamp);
        }

        if let Some(address) = msg.extended_header.address {
            buf.reserve(mem::size_of::<u64>());
            tracing::debug!("Address : {:#064b}", address);
            buf.put_u64(address);
        }

        if let Some(payload) = msg.payload {
            buf.reserve(payload.len());
            tracing::debug!("Payload : {:?}", payload);
            buf.put(payload);
        }

        Ok(())
    }
}

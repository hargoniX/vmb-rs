//! Contains a typified representation of the vmb message.

use crate::types::{Bus, Id, Route};

use bytes::BytesMut;

/// A vmb message.
pub struct Message {
    pub extended_header: ExtendedHeader,
    pub payload: Option<BytesMut>,
}

/// The header together with the optional timestamp and the optional address is called the extended header.
pub struct ExtendedHeader {
    pub header: Header,
    pub timestamp: Option<u32>,
    pub address: Option<u64>,
}

/// The header of a message.
pub struct Header {
    pub r#type: Type,
    pub size: u8,
    pub slot: u8,
    pub id: Id,
}

impl From<[u8; 4]> for Header {
    fn from(header_bytes: [u8; 4]) -> Self {
        Header {
            r#type: Type::from(header_bytes[0]),
            size: header_bytes[1],
            slot: header_bytes[2],
            id: Id::from(header_bytes[3]),
        }
    }
}

/// The TYPE Byte in the Header has the following bits: bus, time, address, route, payload,
/// request, lock, unused.
pub struct Type {
    /// See documentation of `Bus`
    pub bus: Bus,
    /// If this bit is set the four byte header is followed by a four byte time stamp
    /// that may be incremented by the bus and other components to provide timing information.
    pub time: bool,
    /// If this bit is set the four byte header and the optional four byte time stamp
    /// are followed by an eight byte address. if the route bit is not set, ignore SLOT
    /// and determine the receiver from the address
    pub address: bool,
    /// See documentation of `Route`
    pub route: Route,
    /// If the payload bit is set, the extended header is followed by a payload.
    /// Its size as a multiple of octas is given by the SIZE byte as SIZE+1.
    /// This allows from 1 octa up to 256 octas payload = 2k byte.
    /// For an empty payload just don't set the payload bit.
    pub payload: bool,
    /// If this bit is set, the bus will set the SLOT byte to the sender slot
    /// before delivering the message but after determining the receiver (which might be specified by the SLOT).
    /// This allows the receiver to send an answer using the route bit.
    /// If the request bit is set, the sending device will expect an answer.
    /// If there is no receiving device or if the receiving device disconnects
    /// before sending an answer, the bus should provide a dummy answer.
    /// Otherwise nothing happens.
    pub request: bool,
    /// If this bit is set, the bus is locked for exclusive access according to the following
    /// rules: After the bus accepts a message with the lock bit, no other message will be accepted
    /// or delivered until the bus gets unlocked again. The the bus then repeats the following two
    /// steps: 1. It will deliver the message to the receiver. 2. Then it will wait until the
    /// receiver sends a message. The bus is unlocked as soon as a message without the lock bit has
    /// been delivered (after step 1 above). To unlock the bus the final receiver can send a
    /// message with four zero byte (without address and route bit, and id=0, it will be ignored).
    pub lock: bool,
    pub unused: bool,
}

impl From<u8> for Type {
    fn from(byte: u8) -> Self {
        Self {
            bus: Bus::from((byte & 1 << 7) != 0),
            time: (byte & 1 << 6) != 0,
            address: (byte & 1 << 5) != 0,
            route: Route::from((byte & 1 << 4) != 0),
            payload: (byte & 1 << 3) != 0,
            request: (byte & 1 << 2) != 0,
            lock: (byte & 1 << 1) != 0,
            unused: (byte & 1) != 0,
        }
    }
}

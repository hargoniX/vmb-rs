//! Contains a builder for `Message`.

use crate::message::{ExtendedHeader, Header, Message, Type};
use crate::types::{Id, Bus, Route};
use crate::constants::MAX_MESSAGE_SIZE;

use bytes::{BytesMut, BufMut};

#[derive(Clone, Debug, PartialEq, Eq)]
/// A builder pattern struct to create new VMB messages. It provides 2 types of methods:
/// 1. Methods that are new_ prefixed, these ones can be used to create one of the many predefined
///    messages.
/// 2. Methods that aren't new_ prefixed, these ones can be used like with other builder type
///    structs after constructing a new builder using the new() method. Once you are happy with the
///    builder configuration you can use .finalize() in order to obtain your message.
pub struct MessagerBuilder {
    message: Message
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MessageBuilderError {
    /// Gets thrown by `MessageBuilder::route` if the user did not adhere to the rules mentioned in
    /// the documentation of `MessageBuilder::route`.
    RouteError,
    /// Gets thrown by `MessageBuilder::payload` if the user did not adhere to the rules mentioned in
    /// the documentation of `MessageBuilder::payload`.
    /// Furthermore it can be thrown by `writebyte`, `writewyde`, `writetetra`, `replybyte`,
    /// `replywyde`, `replyetra`. If their respective size constraints for the payload (1,2,4) were
    /// not met.
    PayloadError,
    /// This is only thrown by `MessageBuilder::new_interrupt` as of now if the user did not
    /// adehere to the rules mentioned in the docuemtnation of `MessageBuilder::new_interrupt`.
    SlotError
}

impl MessagerBuilder {
    /// Creates a new `MessageBuilder` with as many values set to false or 0 as possible.
    pub fn new() -> Self {
        Self {
            message: Message {
                extended_header: ExtendedHeader {
                    header: Header {
                        r#type: Type {
                            bus: Bus::DeviceMessage,
                            time: false,
                            address: false,
                            route: Route::SlotRoute,
                            payload: false,
                            request: false,
                            lock: false,
                            unused: false
                        },
                        size: 0,
                        slot: 0,
                        id: Id::Other(0),
                    },
                    timestamp: None,
                    address: None,
                },
                payload: None
            }
        }
    }

    /// Destructs the builder and returns the newly created message.
    pub fn finalize(self) -> Message {
        self.message
    }

    /// Constructs an IGNORE message
    /// Note that since an IGNORE message is *always* a device message
    pub fn new_ignore(timestamp: Option<u32>, address: Option<u64>, route: Route, lock: bool, slot: u8) -> Message {
        // This unwrap() is fine since IGNORE messages always have ID set to 0.
        // And thus route() cannot possiby error here.
        let mut builder = MessagerBuilder::new().bus(Bus::DeviceMessage).id(Id::Ignore).route(route).unwrap().slot(slot);

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if let Some(address) = address {
            builder = builder.address(address);
        }

        if lock {
            builder = builder.lock();
        }

        builder.finalize()
    }

    /// Constructs a READ message.
    pub fn new_read(timestamp: Option<u32>, address: u64, lock: bool, slot: u8) -> Message {
        // The unwrap() is fine since setting route to SlotRoute cannot possibly error.
        // The reason route is set to SlotRoute unlike the spec which says "any", is that
        // OtherRoute would be an invalid value since the Id is neither 0 nor is bus set to
        // BusMessage.
        let mut builder = MessagerBuilder::new().bus(Bus::DeviceMessage).address(address).id(Id::Read).request().route(Route::SlotRoute).unwrap().slot(slot);

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if lock {
            builder = builder.lock();
        }

        builder.finalize()
    }

    /// Constructs a WRITE message.
    pub fn new_write(timestamp: Option<u32>, address: u64, lock: bool, slot: u8, payload: BytesMut) -> Result<Message, MessageBuilderError> {
        // The unwrap() is fine since setting route to SlotRoute cannot possibly error.
        // The reason route is set to SlotRoute unlike the spec which says "any", is that
        // OtherRoute would be an invalid value since the Id is neither 0 nor is bus set to
        // BusMessage.
        let mut builder = MessagerBuilder::new().bus(Bus::DeviceMessage).address(address).id(Id::Write).route(Route::SlotRoute).unwrap().slot(slot).payload(payload)?;

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if lock {
            builder = builder.lock();
        }

        Ok(builder.finalize())
    }

    /// Constructs a READREPLY message.
    pub fn new_readreply(timestamp: Option<u32>, address: u64, lock: bool, slot: u8, payload: BytesMut) -> Result<Message, MessageBuilderError> {
        // The unwrap() is fine since setting route to SlotRoute cannot possibly error.
        // The reason route is set to SlotRoute unlike the spec which says "any", is that
        // OtherRoute would be an invalid value since the Id is neither 0 nor is bus set to
        // BusMessage.
        let mut builder = MessagerBuilder::new().bus(Bus::DeviceMessage).address(address).id(Id::Readreply).route(Route::SlotRoute).unwrap().slot(slot).payload(payload)?;

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if lock {
            builder = builder.lock();
        }

        Ok(builder.finalize())
    }

    /// Constructs a NOREPLY message.
    pub fn new_noreply(timestamp: Option<u32>, address: u64, lock: bool, slot: u8) -> Message {
        // The unwrap() is fine since setting route to SlotRoute cannot possibly error.
        // The reason route is set to SlotRoute unlike the spec which says "any", is that
        // OtherRoute would be an invalid value since the Id is neither 0 nor is bus set to
        // BusMessage.
        let mut builder = MessagerBuilder::new().bus(Bus::DeviceMessage).address(address).id(Id::Noreply).route(Route::SlotRoute).unwrap().slot(slot);

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if lock {
            builder = builder.lock();
        }

        builder.finalize()
    }

    fn read_word_helper(timestamp: Option<u32>, address: u64, lock: bool, slot: u8, id: Id) -> Message {
        // The unwrap() is fine since setting route to SlotRoute cannot possibly error.
        // The reason route is set to SlotRoute unlike the spec which says "any", is that
        // OtherRoute would be an invalid value since the Id is neither 0 nor is bus set to
        // BusMessage.
        let mut builder = MessagerBuilder::new().bus(Bus::DeviceMessage).address(address).id(id).route(Route::SlotRoute).unwrap().slot(slot).request();

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if lock {
            builder = builder.lock();
        }

        builder.finalize()
    }

    /// Constructs a READBYTE message.
    pub fn new_readbyte(timestamp: Option<u32>, address: u64, lock: bool, slot: u8) -> Message {
        MessagerBuilder::read_word_helper(timestamp, address, lock, slot, Id::Readbyte)
    }

    /// Constructs a READWYDE message.
    pub fn new_readwyde(timestamp: Option<u32>, address: u64, lock: bool, slot: u8) -> Message {
        MessagerBuilder::read_word_helper(timestamp, address, lock, slot, Id::Readwyde)
    }

    /// Constructs a READTETRA message.
    pub fn new_readtetra(timestamp: Option<u32>, address: u64, lock: bool, slot: u8) -> Message {
        MessagerBuilder::read_word_helper(timestamp, address, lock, slot, Id::Readtetra)
    }

    /// The size checks for the payload have to be done by the calling method still.
    fn write_word_helper(timestamp: Option<u32>, address: u64, payload: BytesMut, lock: bool, slot: u8, id: Id) -> Result<Message, MessageBuilderError> {
        // The unwrap() is fine since setting route to SlotRoute cannot possibly error.
        // The reason route is set to SlotRoute unlike the spec which says "any", is that
        // OtherRoute would be an invalid value since the Id is neither 0 nor is bus set to
        // BusMessage.
        let mut builder = MessagerBuilder::new().bus(Bus::DeviceMessage).address(address).id(id).route(Route::SlotRoute).unwrap().slot(slot);

        // The unwrap() is fine since we did a more specific check for the length above.
        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if lock {
            builder = builder.lock();
        }

        let mut message = builder.finalize();
        message.extended_header.header.size = 0;
        message.extended_header.header.r#type.payload = true;
        message.payload = Some(payload);

        Ok(message)
    }
    /// Constructs a WRITEBYTE message.
    pub fn new_writebyte(timestamp: Option<u32>, address: u64, mut payload: BytesMut, lock: bool, slot: u8) -> Result<Message, MessageBuilderError> {
        if payload.len() != 1 {
            return Err(MessageBuilderError::PayloadError)
        }

        payload.reserve(7);
        payload.put_slice(&[0,0,0,0,0,0,0]);

        MessagerBuilder::write_word_helper(timestamp, address, payload, lock, slot, Id::Writebyte)
    }

    /// Constructs a WRITEWYDE message.
    pub fn new_writewyde(timestamp: Option<u32>, address: u64, mut payload: BytesMut, lock: bool, slot: u8) -> Result<Message, MessageBuilderError> {
        if payload.len() != 2 {
            return Err(MessageBuilderError::PayloadError)
        }
        payload.reserve(6);
        payload.put_slice(&[0,0,0,0,0,0]);

        MessagerBuilder::write_word_helper(timestamp, address, payload, lock, slot, Id::Writewyde)
    }

    /// Constructs a WRITETETRA message.
    pub fn new_writetetra(timestamp: Option<u32>, address: u64, mut payload: BytesMut, lock: bool, slot: u8) -> Result<Message, MessageBuilderError> {
        if payload.len() != 4 {
            return Err(MessageBuilderError::PayloadError)
        }

        payload.reserve(4);
        payload.put_slice(&[0,0,0,0]);

        MessagerBuilder::write_word_helper(timestamp, address, payload, lock, slot, Id::Writetetra)
    }

    /// Constructs a BYTEREPLY message.
    pub fn new_bytereply(timestamp: Option<u32>, address: u64, mut payload: BytesMut, lock: bool, slot: u8) -> Result<Message, MessageBuilderError> {
        if payload.len() != 1 {
            return Err(MessageBuilderError::PayloadError)
        }

        payload.reserve(7);
        payload.put_slice(&[0,0,0,0,0,0,0]);

        MessagerBuilder::write_word_helper(timestamp, address, payload, lock, slot, Id::Bytereply)
    }

    /// Constructs a WYDEREPLY message.
    pub fn new_wydereply(timestamp: Option<u32>, address: u64, mut payload: BytesMut, lock: bool, slot: u8) -> Result<Message, MessageBuilderError> {
        if payload.len() != 2 {
            return Err(MessageBuilderError::PayloadError)
        }

        payload.reserve(6);
        payload.put_slice(&[0,0,0,0,0,0]);

        MessagerBuilder::write_word_helper(timestamp, address, payload, lock, slot, Id::Wydereply)
    }

    /// Constructs a TETRAREPLY message.
    pub fn new_tetrareply(timestamp: Option<u32>, address: u64, mut payload: BytesMut, lock: bool, slot: u8) -> Result<Message, MessageBuilderError> {
        if payload.len() != 4 {
            return Err(MessageBuilderError::PayloadError)
        }

        payload.reserve(4);
        payload.put_slice(&[0,0,0,0]);

        MessagerBuilder::write_word_helper(timestamp, address, payload, lock, slot, Id::Tetrareply)
    }

    /// Constructs a new TERMINATE message.
    pub fn new_terminate() -> Message {
        let builder = MessagerBuilder::new().bus(Bus::BusMessage).id(Id::Terminate).route(Route::OtherRoute).unwrap();

        builder.finalize()
    }

    /// Constructs a new REGISTER message.
    pub fn new_register(timestamp: Option<u32>, lock: bool, slot: u8, payload: BytesMut) -> Result<Message, MessageBuilderError> {
        let mut builder = MessagerBuilder::new().bus(Bus::BusMessage).id(Id::Register).slot(slot).payload(payload)?.route(Route::OtherRoute).unwrap();

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if lock {
            builder = builder.lock();
        }

        Ok(builder.finalize())
    }

    /// Constructs a new UNREGISTER message.
    pub fn new_unregister(timestamp: Option<u32>, lock: bool, slot: u8) -> Message {
        let mut builder = MessagerBuilder::new().bus(Bus::BusMessage).id(Id::Unregister).slot(slot).route(Route::OtherRoute).unwrap();

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        if lock {
            builder = builder.lock();
        }

        builder.finalize()
    }

    /// Constructs a new INTERRUPT message.
    /// Note that INTERRUPTS can only be generated from SLOT 0 to 63.
    pub fn new_interrupt(timestamp: Option<u32>, slot: u8) -> Result<Message, MessageBuilderError> {
        if slot > 63 {
            return Err(MessageBuilderError::SlotError);
        }
        let mut builder = MessagerBuilder::new().bus(Bus::BusMessage).id(Id::Interrupt).slot(slot).route(Route::OtherRoute).unwrap();

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        Ok(builder.finalize())
    }

    /// Constructs a new RESET message.
    pub fn new_reset(timestamp: Option<u32>, slot: u8) -> Message {
        let mut builder = MessagerBuilder::new().bus(Bus::BusMessage).id(Id::Reset).slot(slot).route(Route::OtherRoute).unwrap();

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        builder.finalize()
    }

    /// Constructs a new POWEROFF message.
    pub fn new_poweroff(timestamp: Option<u32>, slot: u8) -> Message {
        let mut builder = MessagerBuilder::new().bus(Bus::BusMessage).id(Id::Poweroff).slot(slot).route(Route::OtherRoute).unwrap();

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        builder.finalize()
    }

    /// Constructs a new POWERON message.
    pub fn new_poweron(timestamp: Option<u32>, slot: u8) -> Message {
        let mut builder = MessagerBuilder::new().bus(Bus::BusMessage).id(Id::Poweron).slot(slot).route(Route::OtherRoute).unwrap();

        if let Some(timestamp) = timestamp {
            builder = builder.timestamp(timestamp);
        }

        builder.finalize()
    }

    /// Set the bus bit in the TYPE part of the header.
    pub fn bus(mut self, bus: Bus) -> Self {
        self.message.extended_header.header.r#type.bus = bus;
        self
    }

    /// Set the time bit in the TYPE part of the header.
    /// Set the timestamp inside the extended header.
    pub fn timestamp(mut self, timestamp: u32) -> Self {
        self.message.extended_header.header.r#type.time = true;
        self.message.extended_header.timestamp = Some(timestamp);
        self
    }

    /// Set the address bit in the TYPE part of the header.
    /// Set the address inside the extended header.
    pub fn address(mut self, address: u64) -> Self {
        self.message.extended_header.header.r#type.address = true;
        self.message.extended_header.address = Some(address);
        self
    }

    /// Sets the route bit in the TYPE part of the header.
    /// Note that:
    /// 1. If you set this to `Route::SlotRoute` you should (obviously) also set a slot.
    /// 2. If you set this to `Route::OtherRoute` you must have set the ID to 0 (default) or
    ///    set the bus bit to `Bus::BusMessage`, otherwise this function will return an error.
    pub fn route(mut self, route: Route) -> Result<Self, MessageBuilderError> {
        if route == Route::OtherRoute {
            if self.message.extended_header.header.id != Id::Other(0) && self.message.extended_header.header.r#type.bus != Bus::BusMessage {
                return Err(MessageBuilderError::RouteError)
            }
        }
        self.message.extended_header.header.r#type.route = route;
        Ok(self)
    }

    /// Sets the payload bit in the TYPE part of the header.
    /// Sets the payload at the end of the message.
    /// Note that:
    /// 1. The length of the payload may not exceed the `MAX_MESSAGE_SIZE`
    /// 2. The payload must be a multiple of 8 bytes long since VMB requires it to be "Octobytes".
    pub fn payload(mut self, payload: BytesMut) -> Result<Self, MessageBuilderError> {
        if payload.len() > MAX_MESSAGE_SIZE as usize || payload.len() % 8 != 0 {
            return Err(MessageBuilderError::PayloadError)
        }
        self.message.extended_header.header.r#type.payload = true;
        self.message.extended_header.header.size = ((payload.len() as u8)/8) - 1;
        self.message.payload = Some(payload);

        Ok(self)
    }

    /// Sets the request bit in the TYPE part of the header.
    /// Note that you should provide your slot as well, otherwise a request type message is
    /// invalid.
    pub fn request(mut self) -> Self {
        self.message.extended_header.header.r#type.request = true;
        self
    }

    /// Sets the lock bit in the TYPE part of the header.
    pub fn lock(mut self) -> Self {
        self.message.extended_header.header.r#type.lock = true;
        self
    }

    /// Sets the SLOT byte of the header.
    pub fn slot(mut self, slot: u8) -> Self {
        self.message.extended_header.header.slot = slot;
        self
    }

    /// Sets the ID byte of the header.
    pub fn id(mut self, id: Id) -> Self {
        self.message.extended_header.header.id = id;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::MessagerBuilder;
    use super::Route;
    use super::Bus;
    use super::Id;
    use std::mem;
    use crate::types::Octa;
    use bytes::{BytesMut, BufMut};

    const TIME_STAMP: Option<u32> = Some(120);
    const ADDRESS: u64 = 0xff;
    const SLOT: u8 = 60;
    const LOCK: bool = true;

    /// Returns a dummy payload plus the SIZE byte for this payload.
    fn dummy_payload(length: usize) -> (BytesMut, usize) {
        let mut payload = BytesMut::with_capacity(length);
        for c in 0..length {
            payload.put_u8(c as u8);
        }
        // Since it is possible to write 1,2 or 4 byte messages
        if length > 4 {
            (payload, (length/8) - 1)
        }
        else {
            (payload, 0)
        }
    }

    /// Pads the provided payload to a length of 8 so it can be used for write{byte,wyde,tetra} and
    /// {bytw,wyde,tetra}reply tests.
    fn pad_payload_to_8(mut payload: BytesMut) -> BytesMut {
        let missing = 8 - payload.len();
        payload.reserve(missing);
        for _ in 0..missing {
            payload.put_u8(0);
        }
        payload
    }

    /// Check that the generated IGNORE message matches the spec.
    #[test]
    fn test_ignore() {
        let message = MessagerBuilder::new_ignore(TIME_STAMP, Some(ADDRESS), Route::from(true), LOCK, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::from(true));
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(0));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, None);
    }

    /// Check that the generated READ message matches the spec.
    #[test]
    fn test_read() {
        let message = MessagerBuilder::new_read(TIME_STAMP, ADDRESS, LOCK, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, true);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(1));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
    }

    /// Check that the generated WRITE message matches the spec.
    #[test]
    fn test_write() {
        let (payload, size) = dummy_payload(10*8);
        let message = MessagerBuilder::new_write(TIME_STAMP, ADDRESS, LOCK, SLOT, payload.clone()).unwrap();
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(2));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), (size+1) * 8);
    }

    /// Check that the generated READREPLY message matches the spec.
    #[test]
    fn test_readreply() {
        let (payload, size) = dummy_payload(10 * 8);
        let message = MessagerBuilder::new_readreply(TIME_STAMP, ADDRESS, LOCK, SLOT, payload.clone()).unwrap();
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(3));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), (size+1) * 8);
    }

    /// Check that the generated NOREPLY message matches the spec.
    #[test]
    fn test_noreply() {
        let message = MessagerBuilder::new_noreply(TIME_STAMP, ADDRESS, LOCK, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(4));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
    }

    /// Check that the generated READBYTE message matches the spec.
    #[test]
    fn test_readbyte() {
        let message = MessagerBuilder::new_readbyte(TIME_STAMP, ADDRESS, LOCK, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, true);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(5));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
    }

    /// Check that the generated READWYDE message matches the spec
    #[test]
    fn test_readwyde() {
        let message = MessagerBuilder::new_readwyde(TIME_STAMP, ADDRESS, LOCK, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, true);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(6));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
    }

    /// Check that the generated READTETRA message matches the spec
    #[test]
    fn test_readtetra() {
        let message = MessagerBuilder::new_readtetra(TIME_STAMP, ADDRESS, LOCK, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, true);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(7));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
    }

    /// Check that the generated WRITEBYTE message matches the spec.
    #[test]
    fn test_writebyte() {
        let (payload, size) = dummy_payload(1);
        let message = MessagerBuilder::new_writebyte(TIME_STAMP, ADDRESS, payload.clone(), LOCK, SLOT).unwrap();
        let payload = pad_payload_to_8(payload);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(8));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), mem::size_of::<Octa>());
    }

    /// Check that the generated WRITEWYDE message matches the spec.
    #[test]
    fn test_writewyde() {
        let (payload, size) = dummy_payload(2);
        let message = MessagerBuilder::new_writewyde(TIME_STAMP, ADDRESS, payload.clone(), LOCK, SLOT).unwrap();
        let payload = pad_payload_to_8(payload);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(9));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), mem::size_of::<Octa>());
    }

    /// Check that the generated WRITETETRA message matches the spec.
    #[test]
    fn test_writetetra() {
        let (payload, size) = dummy_payload(4);
        let message = MessagerBuilder::new_writetetra(TIME_STAMP, ADDRESS, payload.clone(), LOCK, SLOT).unwrap();
        let payload = pad_payload_to_8(payload);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(10));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), mem::size_of::<Octa>());
    }

    /// Check that the generated BYTEREPLY message matches the spec.
    #[test]
    fn test_bytereply() {
        let (payload, size) = dummy_payload(1);
        let message = MessagerBuilder::new_bytereply(TIME_STAMP, ADDRESS, payload.clone(), LOCK, SLOT).unwrap();
        let payload = pad_payload_to_8(payload);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(11));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), mem::size_of::<Octa>());
    }

    /// Check that the generated WYDEREPLY message matches the spec.
    #[test]
    fn test_wydereply() {
        let (payload, size) = dummy_payload(2);
        let message = MessagerBuilder::new_wydereply(TIME_STAMP, ADDRESS, payload.clone(), LOCK, SLOT).unwrap();
        let payload = pad_payload_to_8(payload);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(12));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), mem::size_of::<Octa>());
    }

    /// Check that the generated TETRAREPLY message matches the spec.
    #[test]
    fn test_tetrareply() {
        let (payload, size) = dummy_payload(4);
        let message = MessagerBuilder::new_tetrareply(TIME_STAMP, ADDRESS, payload.clone(), LOCK, SLOT).unwrap();
        let payload = pad_payload_to_8(payload);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(false));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, true);
        assert_eq!(message.extended_header.header.r#type.route, Route::SlotRoute);
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(13));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, Some(ADDRESS));
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), mem::size_of::<Octa>());
    }

    /// Check that the generated TERMINATE message matches the spec.
    #[test]
    fn test_terminate() {
        let message = MessagerBuilder::new_terminate();
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(true));
        assert_eq!(message.extended_header.header.r#type.time, false);
        assert_eq!(message.extended_header.header.r#type.address, false);
        assert_eq!(message.extended_header.header.r#type.route, Route::from(false));
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, false);
        assert_eq!(message.extended_header.header.size, 0);
        assert_eq!(message.extended_header.header.slot, 0);
        assert_eq!(message.extended_header.header.id, Id::from(0xF9));
        assert_eq!(message.extended_header.timestamp, None);
        assert_eq!(message.extended_header.address, None);
    }

    /// Check that the generated REGISTER message matches the spec.
    #[test]
    fn test_register() {
        let (payload, size) = dummy_payload(32);
        let message = MessagerBuilder::new_register(TIME_STAMP, LOCK, SLOT, payload.clone()).unwrap();
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(true));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, false);
        assert_eq!(message.extended_header.header.r#type.route, Route::from(false));
        assert_eq!(message.extended_header.header.r#type.payload, true);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.size, size as u8);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(0xFA));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, None);
        assert_eq!(message.payload, Some(payload.clone()));
        assert_eq!(message.payload.unwrap().len(), (size+1) * 8);
    }

    /// Check that the generated UNREGISTER message matches the spec.
    #[test]
    fn test_unregister() {
        let message = MessagerBuilder::new_unregister(TIME_STAMP, LOCK, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(true));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, false);
        assert_eq!(message.extended_header.header.r#type.route, Route::from(false));
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, LOCK);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(0xFB));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, None);
    }

    /// Check that the generated INTERRUPT message matches the spec.
    #[test]
    fn test_interrupt() {
        let message = MessagerBuilder::new_interrupt(TIME_STAMP, SLOT).unwrap();
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(true));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, false);
        assert_eq!(message.extended_header.header.r#type.route, Route::from(false));
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, false);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(0xFC));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, None);
    }

    /// Check that the generated RESET message matches the spec.
    #[test]
    fn test_reset() {
        let message = MessagerBuilder::new_reset(TIME_STAMP, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(true));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, false);
        assert_eq!(message.extended_header.header.r#type.route, Route::from(false));
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, false);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(0xFD));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, None);
    }

    /// Check that the generated POWEROFF message matches the spec.
    #[test]
    fn test_poweroff() {
        let message = MessagerBuilder::new_poweroff(TIME_STAMP, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(true));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, false);
        assert_eq!(message.extended_header.header.r#type.route, Route::from(false));
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, false);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(0xFE));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, None);
    }

    /// Check that the generated POWERON message matches the spec.
    #[test]
    fn test_poweron() {
        let message = MessagerBuilder::new_poweron(TIME_STAMP, SLOT);
        assert_eq!(message.extended_header.header.r#type.bus, Bus::from(true));
        assert_eq!(message.extended_header.header.r#type.time, true);
        assert_eq!(message.extended_header.header.r#type.address, false);
        assert_eq!(message.extended_header.header.r#type.route, Route::from(false));
        assert_eq!(message.extended_header.header.r#type.payload, false);
        assert_eq!(message.extended_header.header.r#type.request, false);
        assert_eq!(message.extended_header.header.r#type.lock, false);
        assert_eq!(message.extended_header.header.slot, SLOT);
        assert_eq!(message.extended_header.header.id, Id::from(0xFF));
        assert_eq!(message.extended_header.timestamp, TIME_STAMP);
        assert_eq!(message.extended_header.address, None);
    }
}

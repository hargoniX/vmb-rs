//! Types used for the vmb protocol
//!
use crate::constants::id;

/// What MMIX calls an u8.
pub type Byte = u8;
/// What MMIX calls an u16.
pub type Wyde = u16;
/// What MMIX calls an u32.
pub type Tetra = u32;
/// What MMIX calls an u64.
pub type Octa = u64;

/// A wrapper around the ID byte of the message header.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Id {
    /// The sender of such a message can use the route bit and the SLOT, or the address bit and the
    /// address field to determine the receiver. If neither is specified the bus will ignore this
    /// message (but may unlock). The receiver can safely ignore this message.
    Ignore,
    /// The sender of this message will expect a answer containing (SIZE+1)*8 byte at the given
    /// address. The answer should have ID = ID_READREPLY (or ID=ID_NOREPLY in case of an error).
    /// The bus will replace the SLOT byte by the sending slot number, such that the receiver of the
    /// message can send an answer using the route bit. The bus will make sure that eventually the
    /// sender will receive an answer. The receiver should send a answer with ID=ID_READREPLY to the
    /// device given in the SLOT byte using the route bit. It should add SIZE+1 octas of payload
    /// containing the memory content starting at the given address.
    Read,
    /// The sender of this message will expect that the SIZE+1 octas contained in the payload will
    /// be stored by the receiver at the given address. The receiver will store the data. There is no answer.
    Write,
    /// The sender of this message will inform the receiver that the SIZE+1 octas contained in the payload
    /// are stored at the given address. The receiver can use the data.
    Readreply,
    /// The sender of this message will inform the receiver that the read request just received can not
    /// be answered. The sender may be either a device or the bus. Typically the sender of such a message
    /// should also raise an interrupt to cause proper error treatment. The receiver should treat this
    /// message as a (soft) read error. It can also ignore this message and process the interrupt caused
    /// by an unsuccessful read.
    Noreply,
    /// The sender of this message will expect a answer containing 1 (READBYTE), 1 (READWYDE) or 4
    /// (READTETRA) byte at the given address. The answer should have IDID_BYTEREPLY,
    /// ID=ID_WYDEREPLY, or ID=ID_TETRAREPLY (or ID=ID_NOREPLY in case of an error). The bus will
    /// replace the SLOT byte by the sending slot number, such that the receiver of the message can
    /// send an answer using the route bit. The bus will make sure that eventually the sender will
    /// receive an answer. The receiver should send a answer with ID=ID_BYTEREPLY, ID=ID_WYDEREPLY,
    /// or ID=ID_TETRAREPLY to the device given in the SLOT byte using the route bit. It should add
    /// 1 octa of payload containing left justified the memory content (1, 2, or 4 byte) starting
    /// at the given address.
    Readbyte,
    /// The sender of this message will expect a answer containing 1 (READBYTE), 1 (READWYDE) or 4
    /// (READTETRA) byte at the given address. The answer should have IDID_BYTEREPLY,
    /// ID=ID_WYDEREPLY, or ID=ID_TETRAREPLY (or ID=ID_NOREPLY in case of an error). The bus will
    /// replace the SLOT byte by the sending slot number, such that the receiver of the message can
    /// send an answer using the route bit. The bus will make sure that eventually the sender will
    /// receive an answer. The receiver should send a answer with ID=ID_BYTEREPLY, ID=ID_WYDEREPLY,
    /// or ID=ID_TETRAREPLY to the device given in the SLOT byte using the route bit. It should add
    /// 1 octa of payload containing left justified the memory content (1, 2, or 4 byte) starting
    /// at the given address.
    Readwyde,
    /// The sender of this message will expect a answer containing 1 (READBYTE), 1 (READWYDE) or 4
    /// (READTETRA) byte at the given address. The answer should have IDID_BYTEREPLY,
    /// ID=ID_WYDEREPLY, or ID=ID_TETRAREPLY (or ID=ID_NOREPLY in case of an error). The bus will
    /// replace the SLOT byte by the sending slot number, such that the receiver of the message can
    /// send an answer using the route bit. The bus will make sure that eventually the sender will
    /// receive an answer. The receiver should send a answer with ID=ID_BYTEREPLY, ID=ID_WYDEREPLY,
    /// or ID=ID_TETRAREPLY to the device given in the SLOT byte using the route bit. It should add
    /// 1 octa of payload containing left justified the memory content (1, 2, or 4 byte) starting
    /// at the given address.
    Readtetra,
    /// The sender of this message will expect that 1, 2 or 4 byte contained left justified in the payload
    /// will be stored by the receiver at the given address. The receiver will store the data. There is no answer.
    Writebyte,
    /// The sender of this message will expect that 1, 2 or 4 byte contained left justified in the payload
    /// will be stored by the receiver at the given address. The receiver will store the data. There is no answer.
    Writewyde,
    /// The sender of this message will expect that 1, 2 or 4 byte contained left justified in the payload
    /// will be stored by the receiver at the given address. The receiver will store the data. There is no answer.
    Writetetra,
    /// The sender of this message will inform the receiver that the 1, 2, or 4 byte contained left
    /// justified in the payload octa are stored at the given address.
    /// The receiver can use the data.
    Bytereply,
    /// The sender of this message will inform the receiver that the 1, 2, or 4 byte contained left
    /// justified in the payload octa are stored at the given address.
    /// The receiver can use the data.
    Wydereply,
    /// The sender of this message will inform the receiver that the 1, 2, or 4 byte contained left
    /// justified in the payload octa are stored at the given address.
    /// The receiver can use the data.
    Tetrareply,
    /// Sending this message is a polite request to terminate the device simulator. The motherboard will
    /// send this message to all connected devices before it terminates. This allows to terminate the
    /// complete device configuration by terminating the motherboard.
    /// Of course it is possible to ignore this message. For example, the VMB IDE application will not
    /// terminate when receiving this message.
    Terminate,
    /// The sender of this message tries to register itself with the bus.
    /// The payload will contain in this order:
    /// * the address (8 byte)
    /// * the limit (8 byte)
    /// * the interrupt mask (8byte)
    /// * the name (a multiple of 8 byte)
    /// * optional: the version number (4 byte major, 4 byte minor)
    /// * optional: further information yet unspecified.
    ///
    /// The SIZE byte should correctly specify the amount of payload supplied which varies because of the name.
    /// The registered device claims to be responsible for the memory range starting at the given address up to
    /// but not including the limit. Any interrupt, where interrupt numbers range between 0 and 63, that is raised
    /// will be delivered to this device if the corresponding bit in the interrupt mask is set.
    /// The name that follows in the payload is a zero terminated string left justified in the payload.
    /// The payload may be longer than the string because the payload is always a multiple of 8 byte.
    /// The name of the device is just for information. The next 8 byte, if they exist, contain the version number of the device.
    ///
    /// Currently, a device can register only once.
    /// It is planed to change this so that a device can register and unregister for multiple memory ranges,
    /// possibly occupying multiple slots, but using only a single TCP/IP connection.
    Register,
    /// A registered device should unregister itself before disconnecting.
    Unregister,
    /// The sender of this message raises an interrupt. The interrupt number should be a number between 0
    /// and 63 nd is contained in the SLOT byte. The bus will check all registered devices and if the
    /// corresponding bit in the interrupt mask is set, it will forward this message to the device.
    /// The receiver of this message should take note of the interrupt and process it.
    Interrupt,
    /// This is the hardware reset signal. This is not a software interrupt. Even if the software of a device is
    /// spinning in a loop or got stuck otherwise the receiver should be able to process this message and get back
    /// into a sane initial state. Devices typically do not send this signal. It is send out by the motherboard/bus.
    Reset,
    /// This is not a power fail interrupt, or something that a device can handle; its definitely the end of power.
    /// If the device has no (virtual) battery it should stop operating. The power off signal will only come after a
    /// power on signal.
    Poweroff,
    /// This is signal is the beginning of life for a device. The device should wait after registering until it
    /// receives the power on signal. Then it should start working.
    Poweron,
    /// A not predefined message ID.
    Other(u8),
}

impl From<u8> for Id {
    fn from(byte: u8) -> Self {
        match byte {
            id::IGNORE => Self::Ignore,
            id::READ => Self::Read,
            id::WRITE => Self::Write,
            id::READREPLY => Self::Readreply,
            id::NOREPLY => Self::Noreply,
            id::READBYTE => Self::Readbyte,
            id::READWYDE => Self::Readwyde,
            id::READTETRA => Self::Readtetra,
            id::WRITEBYTE => Self::Writebyte,
            id::WRITEWYDE => Self::Writewyde,
            id::WRITETETRA => Self::Writetetra,
            id::BYTEREPLY => Self::Bytereply,
            id::WYDEREPLY => Self::Wydereply,
            id::TETRAREPLY => Self::Tetrareply,
            id::TERMINATE => Self::Terminate,
            id::REGISTER => Self::Register,
            id::UNREGISTER => Self::Unregister,
            id::INTERRUPT => Self::Interrupt,
            id::RESET => Self::Reset,
            id::POWEROFF => Self::Poweroff,
            id::POWERON => Self::Poweron,
            other => Self::Other(other),
        }
    }
}

impl Into<u8> for Id {
    fn into(self) -> u8 {
        match self {
            Self::Ignore => id::IGNORE,
            Self::Read => id::READ,
            Self::Write => id::WRITE,
            Self::Readreply => id::READREPLY,
            Self::Noreply => id::NOREPLY,
            Self::Readbyte => id::READBYTE,
            Self::Readwyde => id::READWYDE,
            Self::Readtetra => id::READTETRA,
            Self::Writebyte => id::WRITEBYTE,
            Self::Writewyde => id::WRITEWYDE,
            Self::Writetetra => id::WRITETETRA,
            Self::Bytereply => id::BYTEREPLY,
            Self::Wydereply => id::WYDEREPLY,
            Self::Tetrareply => id::TETRAREPLY,
            Self::Terminate => id::TERMINATE,
            Self::Register => id::REGISTER,
            Self::Unregister => id::UNREGISTER,
            Self::Interrupt => id::INTERRUPT,
            Self::Reset => id::RESET,
            Self::Poweroff => id::POWEROFF,
            Self::Poweron => id::POWERON,
            Self::Other(val) => val,
        }
    }
}

/// A wrapper around the bus bit of the TYPE part of the message header.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bus {
    /// This is a message from device to device which just needs to be forwarded by the bus.
    DeviceMessage,
    /// This is a message for/from the bus (motherboard). The bus will handle this message.
    /// The type of the message is determined by the ID byte.
    BusMessage,
}

impl From<bool> for Bus {
    fn from(bit: bool) -> Self {
        match bit {
            false => Self::DeviceMessage,
            true => Self::BusMessage,
        }
    }
}

impl Into<bool> for Bus {
    fn into(self) -> bool {
        match self {
            Self::DeviceMessage => false,
            Self::BusMessage => true,
        }
    }
}

/// A wrapper around the route bit of the TYPE part of the message header.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Route {
    /// If this bit is set, send the message to the SLOT byte.
    SlotRoute,
    /// If the address bit is set, determine the receiver from the address,
    /// otherwise there is no receiver, which may be OK for bus messages,
    /// or if the ID is 0 (ignore)
    OtherRoute,
}

impl From<bool> for Route {
    fn from(bit: bool) -> Self {
        match bit {
            false => Self::OtherRoute,
            true => Self::SlotRoute,
        }
    }
}

impl Into<bool> for Route {
    fn into(self) -> bool {
        match self {
            Self::OtherRoute => false,
            Self::SlotRoute => true,
        }
    }
}

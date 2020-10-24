//! Constants associated with the vmb protocol

/// The message size can be determined from the first two byte (the TYPE and the
/// SIZE byte of the header) of the message. It is computed as follows:
/// The minimum size of the message is 4 byte. (the message consists of just the header)
/// Add 4, if the time bit in the TYPE byte is set.
/// Add 8, if the address bit in the TYPE byte is set.
/// Add 8*(SIZE+1), if the payload bit in the TYPE byte is set.
pub const MAX_MESSAGE_SIZE: u16 = 4 + 4 + 8 + 8 * 256;
/// The minimum size of the message is 4 byte. (the message consists of just the header)
pub const MIN_MESSAGE_SIZE: u16 = 4;

pub mod id {
    pub const IGNORE: u8 = 0;
    pub const READ: u8 = 1;
    pub const WRITE: u8 = 2;
    pub const READREPLY: u8 = 3;
    pub const NOREPLY: u8 = 4;
    pub const READBYTE: u8 = 5;
    pub const READWYDE: u8 = 6;
    pub const READTETRA: u8 = 7;
    pub const WRITEBYTE: u8 = 8;
    pub const WRITEWYDE: u8 = 9;
    pub const WRITETETRA: u8 = 10;
    pub const BYTEREPLY: u8 = 11;
    pub const WYDEREPLY: u8 = 12;
    pub const TETRAREPLY: u8 = 13;
    pub const TERMINATE: u8 = 0xF9;
    pub const REGISTER: u8 = 0xFA;
    pub const UNREGISTER: u8 = 0xFB;
    pub const INTERRUPT: u8 = 0xFC;
    pub const RESET: u8 = 0xFD;
    pub const POWEROFF: u8 = 0xFE;
    pub const POWERON: u8 = 0xFF;
}

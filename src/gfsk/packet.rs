#[derive(Debug)]
#[repr(C)]
pub struct Packet {
    pub crc_match: bool,
    pub data: [u8; 34]
}

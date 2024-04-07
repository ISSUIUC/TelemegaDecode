

extern "C" {
    // allegedly Option<extern fn> is a nullable ptr
    fn ao_fec_decode(r#in: *const u8, len: u16, out: *mut u8, out_len: u8, callback: Option<extern fn() -> u16>) -> u8;
}

pub const FEC_DECODE_CRC_OK: u8 = 0x80;

pub fn fec_decode(input: &[u8], out: &mut [u8]) {
    unsafe {
        ao_fec_decode(input.as_ptr(), input.len() as u16, out.as_mut_ptr(), out.len() as u8, None);
    }
}
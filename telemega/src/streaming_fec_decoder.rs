use crate::ao;
use crate::packet::Packet;

const SYNC_PATTERN: [u8; 34] = to_bits(b"1010101010101010101101001110010001");
const MESSAGE_LEN: usize = 72 * 8;

enum FecDecoderState {
    SYNCING,
    BUFFERING
}

pub struct StreamingFecDecoder {
    sync_buffer: [u8; SYNC_PATTERN.len()],
    message_buffer: [u8; MESSAGE_LEN],
    message_head: usize,
    state: FecDecoderState
}


impl StreamingFecDecoder {
    pub fn new() -> StreamingFecDecoder {
        StreamingFecDecoder {
            sync_buffer: [0xff; SYNC_PATTERN.len()],
            message_buffer: [0; MESSAGE_LEN],
            message_head: 0,
            state: FecDecoderState::SYNCING
        }
    }

    pub fn feed(&mut self, sample: bool, for_each: impl FnMut(Packet)) {
        match self.state {
            FecDecoderState::SYNCING => {
                self.sync_buffer.copy_within(1.., 0);
                self.sync_buffer[self.sync_buffer.len()-1] = sample as u8;

                if self.sync_buffer == SYNC_PATTERN {
                    self.state = FecDecoderState::BUFFERING;
                }
            }
            FecDecoderState::BUFFERING => {
                self.message_buffer[self.message_head] = if sample { 0x00 } else { 0xff };
                self.message_head += 1;
                if self.message_head == MESSAGE_LEN {
                    self.process_message(for_each);
                    self.state = FecDecoderState::SYNCING;
                    self.message_head = 0;
                }
            }
        }
    }

    fn process_message(&self, mut for_each: impl FnMut(Packet)) {
        let mut decoded = [0; 34];
        ao::fec_decode(&self.message_buffer, &mut decoded);
        let crc_match = decoded[decoded.len() - 1] == ao::FEC_DECODE_CRC_OK;
        for_each(Packet { crc_match, data: decoded })
    }
}


const fn to_bits<const N: usize>(s: &[u8]) -> [u8; N] {
    let mut new = [0; N];
    let mut i = 0;
    while i < N {
        new[i] = if s[i] == b'0' {
            0
        } else {
            1
        };
        i += 1;
    }
    new
}
use std::fs::File;
use std::io::{BufWriter, Write};
use num_complex::Complex;
use ringbuffer::{AllocRingBuffer, RingBuffer};
use crate::{ao, Packet};
use crate::shifter::Shifter;

const BUFFER_SIZE: usize = 1024 * 32;
const OFFSET_SIZE: usize = 64;
const SYNC_PATTERN: &[u8] = "1010101010101010101101001110010001".as_bytes();
const MESSAGE_BITS: usize = 72 * 8;
const SYNC_BITS: usize = SYNC_PATTERN.len();
const TOTAL_PACKET_BITS: usize = SYNC_BITS + MESSAGE_BITS;
const AVERAGING_WIDTH: usize = 128;
pub struct StreamingGFSKDecoder {
    buffer: Vec<Complex<f32>>,

    total_idx: usize,

    previous_samples: [Complex<f32>;OFFSET_SIZE],
    staging: [Complex<f32>;OFFSET_SIZE],

    zi: [[Complex<f32>; 2]; 3],

    shifter: Shifter,

    running_sum: f32,
    avg_buff: [f32;AVERAGING_WIDTH],
    avg_ring: AllocRingBuffer<f32>,
    bit_width: f64,
}

impl StreamingGFSKDecoder {
    pub fn new(sample_rate: f64, center: f64, baud: f64) -> StreamingGFSKDecoder {
        StreamingGFSKDecoder {
            buffer: Vec::with_capacity(BUFFER_SIZE),
            total_idx: 0,
            previous_samples: [Complex::new(0.0, 0.0); OFFSET_SIZE],
            staging: [Complex::new(0.0, 0.0); OFFSET_SIZE],
            zi: [[Complex::new(0.0, 0.0); 2]; 3],
            shifter: Shifter::new(-center, sample_rate),
            avg_buff: [0f32;AVERAGING_WIDTH],
            bit_width: sample_rate/baud,
            running_sum: 0f32,
            avg_ring: AllocRingBuffer::new(TOTAL_PACKET_BITS * (sample_rate/baud).ceil() as usize),
        }
    }

    pub fn feed(&mut self, mut item: &[Complex<f32>], mut for_each: impl FnMut(Packet)) {
        loop {
            let needed = BUFFER_SIZE - self.buffer.len();
            if item.len() > needed {
                self.buffer.extend_from_slice(&item[..needed]);
                item = &item[needed..];
                if self.buffer.len() == BUFFER_SIZE {
                    self.process_buffer(&mut for_each);
                }
            } else {
                self.buffer.extend_from_slice(item);
                if self.buffer.len() == BUFFER_SIZE {
                    self.process_buffer(&mut for_each);
                }
                return;
            }
        }
    }

    #[allow(unused)]
    pub fn finish(&mut self, for_each: impl FnMut(Packet)) {
        if self.buffer.len() < OFFSET_SIZE {
            self.process_buffer(for_each);
        }
    }

    fn check_ring(&mut self) -> Option<Packet> {
        if self.avg_ring.len() < (TOTAL_PACKET_BITS as f64 * self.bit_width) as usize {
            return None;
        }
        for i in 0..SYNC_BITS {
            let idx = (self.bit_width * i as f64) as usize;
            //sync pattern is inversed
            let bit = self.avg_ring[idx] < 0.0;

            if (SYNC_PATTERN[i] != '0' as u8) != bit {
                return None;
            }
        }

        let mut message = [0u8;MESSAGE_BITS];

        for i in 0..MESSAGE_BITS {
            let idx = (self.bit_width * (i + SYNC_BITS) as f64) as usize;
            let bit = self.avg_ring[idx] > 0.0;
            message[i] = if bit { 0xff } else { 0x00 };
        }

        let mut data = [0; 34];
        ao::fec_decode(&message, &mut data);
        let crc_match = data[data.len() - 1] == ao::FEC_DECODE_CRC_OK;
        return Some(Packet{crc_match, data});
    }

    fn process_buffer(&mut self, mut for_each: impl FnMut(Packet)) {
        if self.buffer.len() < OFFSET_SIZE {
            panic!("Error: Buffer too small.");
        }

        self.shifter.shift(&mut self.buffer, self.total_idx);
        sosfilt(&LOW_PASS_SOS_33K, &mut self.buffer, &mut self.zi);
        self.staging.copy_from_slice(&self.buffer[self.buffer.len() - OFFSET_SIZE..]);
        polar_discriminate(&mut self.buffer, &self.previous_samples);

        //periodically reset running sum to avoid floating point error buildup
        self.running_sum = self.avg_buff.iter().sum();
        for i in 0..self.buffer.len() {
            let s = self.buffer[i];
            let im = s.im;
            let idx = (self.total_idx + i) % self.avg_buff.len();
            self.running_sum = self.running_sum + im - self.avg_buff[idx];
            self.avg_buff[idx] = im;
            self.avg_ring.push(self.running_sum / AVERAGING_WIDTH as f32);
            if let Some(packet) = self.check_ring() {
                if packet.crc_match {
                    for_each(packet);
                    self.avg_ring.clear();
                }
            }
        }
        std::mem::swap(&mut self.previous_samples, &mut self.staging);

        self.total_idx += self.buffer.len();
        self.buffer.clear();
    }
}


const LOW_PASS_SOS_33K: [[f32; 6]; 3] = [
    [3.869494405731452e-12, 7.738988811462904e-12, 3.869494405731452e-12, 1.0, -0.989582475318754, 0.0],
    [1.0, 2.0, 1.0, 1.0, -1.98308989599488, 0.9831986360344092],
    [1.0, 1.0, 0.0, 1.0, -1.9934396492520414, 0.9935489568062257],
];


fn sosfilt<const N: usize>(sos: &[[f32; 6]; N], x: &mut [Complex<f32>], zi: &mut [[Complex<f32>; 2]; N]) {
    for i in 0..x.len() {
        let mut x_c = x[i];
        for j in 0..N {
            let section = &sos[j];
            let x_n = section[0] * x_c + zi[j][0];
            zi[j][0] = section[1] * x_c - section[4] * x_n + zi[j][1];
            zi[j][1] = section[2] * x_c - section[5] * x_n;
            x_c = x_n;
        }
        x[i] = x_c;
    }
}

fn polar_discriminate(x: &mut [Complex<f32>], prev: &[Complex<f32>;OFFSET_SIZE]) {
    for i in (prev.len()..x.len()).rev() {
        x[i] = x[i-prev.len()] * x[i].conj();
    }
    for i in 0..prev.len() {
        x[i] = prev[i] * x[i].conj();
    }
}
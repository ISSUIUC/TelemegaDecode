use num_complex::Complex;
use crate::shifter::Shifter;
use crate::transition::Transition;

const BUFFER_SIZE: usize = 1024 * 32;
const OFFSET_SIZE: usize = 64;

pub struct StreamingGFSKDecoder {
    // file: File,

    buffer: Vec<Complex<f32>>,
    bits: Vec<bool>,

    total_idx: usize,

    previous_samples: [Complex<f32>;OFFSET_SIZE],
    staging: [Complex<f32>;OFFSET_SIZE],

    prev_bit: bool,

    zi: [[Complex<f32>; 2]; 3],

    shifter: Shifter,
}

impl StreamingGFSKDecoder {
    pub fn new(sample_rate: f64, center: f64) -> StreamingGFSKDecoder {
        StreamingGFSKDecoder {
            buffer: Vec::with_capacity(BUFFER_SIZE),
            bits: Vec::with_capacity(BUFFER_SIZE),
            total_idx: 0,
            previous_samples: [Complex::new(0.0, 0.0); OFFSET_SIZE],
            staging: [Complex::new(0.0, 0.0); OFFSET_SIZE],
            prev_bit: false,
            zi: [[Complex::new(0.0, 0.0); 2]; 3],
            shifter: Shifter::new(-center, sample_rate),
        }
    }

    pub fn feed(&mut self, mut item: &[Complex<f32>], mut for_each: impl FnMut(Transition)) {
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
    pub fn finish(&mut self, for_each: impl FnMut(Transition)) {
        if self.buffer.len() < OFFSET_SIZE {
            self.process_buffer(for_each);
        }
    }

    fn process_buffer(&mut self, mut for_each: impl FnMut(Transition)) {
        if self.buffer.len() < OFFSET_SIZE {
            panic!("Error: Buffer too small.");
        }

        self.shifter.shift(&mut self.buffer, self.total_idx);
        sosfilt(&LOW_PASS_SOS_33K, &mut self.buffer, &mut self.zi);
        self.staging.copy_from_slice(&self.buffer[self.buffer.len() - OFFSET_SIZE..]);
        polar_discriminate(&mut self.buffer, &self.previous_samples);
        std::mem::swap(&mut self.previous_samples, &mut self.staging);

        self.bits.clear();
        self.bits.extend(self.buffer.iter().map(|x| x.im < 0.0));

        if self.prev_bit != self.bits[0] {
            for_each(Transition { idx: self.total_idx, new_state: self.bits[0] });
        }
        for i in 0..self.buffer.len()-1 {
            if self.bits[i] != self.bits[i+1] {
                for_each(Transition { idx: i + 1 + self.total_idx, new_state: self.bits[i+1] });
            }
        }
        self.prev_bit = self.bits[self.buffer.len()-1];
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
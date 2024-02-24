use std::collections::VecDeque;
use std::fs::File;
use num_complex::Complex;
use crate::shifter::Shifter;
use crate::transition::Transition;
use std::io::Write;

const BUFFER_SIZE: usize = 1024 * 32;

pub struct StreamingGFSKDecoder {
    file: File,
    out: VecDeque<Transition>,

    buffer: Vec<Complex<f64>>,
    bits: Vec<bool>,

    total_idx: usize,

    previous_samples: Vec<Complex<f64>>,
    staging: Vec<Complex<f64>>,

    prev_bit: bool,

    zi: [[Complex<f64>; 2]; 3],

    shifter: Shifter,
    offset: f64,
    offset_buffer_size: usize,
}

impl StreamingGFSKDecoder {
    pub fn new(sample_rate: f64, center: f64) -> StreamingGFSKDecoder {
        let offset = (sample_rate / center / 4.0).abs();
        let offset_buffer_size = 1 + offset.abs().ceil() as usize;

        StreamingGFSKDecoder {
            file: File::create("log.dat").unwrap(),
            out: VecDeque::with_capacity(BUFFER_SIZE),
            buffer: Vec::with_capacity(BUFFER_SIZE),
            bits: Vec::with_capacity(BUFFER_SIZE),
            total_idx: 0,
            previous_samples: vec![Complex::new(0.0, 0.0); offset_buffer_size],
            staging: vec![Complex::new(0.0, 0.0); offset_buffer_size],
            prev_bit: false,
            zi: [[Complex::new(0.0, 0.0); 2]; 3],
            shifter: Shifter::new(-center, sample_rate),
            offset, offset_buffer_size,
        }
    }

    pub fn feed(&mut self, item: Complex<f64>, for_each: impl FnMut(Transition)) {
        self.buffer.push(item);
        if self.buffer.len() == BUFFER_SIZE {
            self.process_buffer(for_each);
        }
    }

    pub fn finish(&mut self, for_each: impl FnMut(Transition)) {
        if self.buffer.len() < self.offset_buffer_size {
            self.process_buffer(for_each);
        }
    }

    fn process_buffer(&mut self, mut for_each: impl FnMut(Transition)) {
        if self.buffer.len() < self.offset_buffer_size {
            panic!("Error: Buffer too small.");
        }

        self.shifter.shift(&mut self.buffer, self.total_idx);
        // sosfilt(&LOW_PASS_SOS_33K, &mut self.buffer, &mut self.zi);
        sosfilt_fast(&LOW_PASS_SOS_33K, &mut self.buffer, &mut self.zi);
        self.staging.copy_from_slice(&self.buffer[self.buffer.len() - self.offset_buffer_size..]);
        polar_discriminate(&mut self.buffer, &self.previous_samples, self.offset);
        // let mut faster = vec![];
        // for cf in &self.buffer {
        //     self.file.write_all(&cf.re.to_le_bytes()).unwrap();
        //     self.file.write_all(&cf.im.to_le_bytes()).unwrap();
        // }

        // writeln!(self.file, "").unwrap();
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
        self.total_idx += self.buffer.len();
        self.buffer.clear();
    }
}


const LOW_PASS_SOS_33K: [[f64; 6]; 3] = [
    [3.869494405731452e-12, 7.738988811462904e-12, 3.869494405731452e-12, 1.0, -0.989582475318754, 0.0],
    [1.0, 2.0, 1.0, 1.0, -1.98308989599488, 0.9831986360344092],
    [1.0, 1.0, 0.0, 1.0, -1.9934396492520414, 0.9935489568062257],
];


fn sosfilt<const N: usize>(sos: &[[f64; 6]; N], x: &mut [Complex<f64>], zi: &mut [[Complex<f64>; 2]; N]) {
    for i in 0..x.len() {
        let mut x_c = x[i];
        for j in 0..N {
            let section = &sos[j];
            // let [zi_0, zi_1] = zi[j];
            let x_n = section[0] * x_c + zi[j][0];
            zi[j][0] = section[1] * x_c - section[4] * x_n + zi[j][1];
            zi[j][1] = section[2] * x_c - section[4] * x_n;
            x_c = x_n;
        }
        x[i] = x_c;
    }
}

fn sosfilt_fast(sos: &[[f64; 6]; 3], x: &mut [Complex<f64>], zi: &mut [[Complex<f64>; 2]; 3]) {
    for i in 0..x.len() {
        let x_c_0 = x[i];

        let section1 = &sos[0];
        let section2 = &sos[1];
        let section3 = &sos[2];

        let zi_0 = &zi[0];
        let zi_1 = &zi[1];
        let zi_2 = &zi[2];

        let mut cp_zi_0 = [Complex::new(0.0, 0.0); 2];
        let mut cp_zi_1 = [Complex::new(0.0, 0.0); 2];
        let mut cp_zi_2 = [Complex::new(0.0, 0.0); 2];

        let x_0 = section1[0] * x_c_0 + zi_0[0];
        let x_1 = section2[0] * x_0 + zi_1[0];
        let x_2 = section3[0] * x_1 + zi_2[0];

        cp_zi_0[0] = section1[1] * x_c_0 - section1[4] * x_0 + zi_0[1];
        cp_zi_1[0] = section2[1] * x_0 - section2[4] * x_1 + zi_1[1];
        cp_zi_2[0] = section3[1] * x_1 - section3[4] * x_2 + zi_2[1];

        cp_zi_0[1] = section1[2] * x_c_0 - section1[5] * x_0;
        cp_zi_1[1] = section2[2] * x_0 - section2[5] * x_1;
        cp_zi_2[1] = section3[2] * x_1 - section3[5] * x_2;

        x[i] = x_2;

        zi[0][0] = cp_zi_0[0];
        zi[1][0] = cp_zi_1[0];
        zi[2][0] = cp_zi_2[0];

        zi[0][1] = cp_zi_0[1];
        zi[1][1] = cp_zi_1[1];
        zi[2][1] = cp_zi_2[1];
    }
}

fn linear_sample(x: &[Complex<f64>], prev: &[Complex<f64>], u: f64) -> Complex<f64> {
    if u >= 0.0 {
        let i_u = u as usize;
        let fract = u.fract();
        let left = x[i_u];
        let right = x[i_u + 1];
        left * (1.0 - fract) + right * fract
    } else if u >= -1.0 {
        let fract = u + 1.0;
        let left = prev[prev.len()-1];
        let right = x[0];
        left * (1.0 - fract) + right * fract
    } else {
        let i_u = -u as usize;
        let fract = -(i_u as f64 + u);
        let left = prev[prev.len() - i_u - 1];
        let right = prev[prev.len() - i_u];

        left * fract + right * (1.0 - fract)
    }
}

fn polar_discriminate(x: &mut [Complex<f64>], prev: &[Complex<f64>], offset: f64) {
    for i in (0..x.len()).rev() {
        x[i] = linear_sample(x, prev, i as f64 - offset) * x[i].conj();
    }
}
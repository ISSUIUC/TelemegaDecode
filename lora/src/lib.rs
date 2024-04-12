use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;
use num_complex::Complex;
use itertools::Itertools;
use rustfft::FftPlanner;
use crate::chirp::{downchirp, upchirp};
use crate::demodulate::LoraDemodulator;
use crate::hamming::{hamming48_decode, hamming48_encode};
use crate::iq_source::{HackRFIQSource, IQSource};
use crate::shifter::Shifter;
use crate::sos::sosfilt;

mod hamming;
mod shifter;
mod sos;
mod chirp;
mod iq_source;
mod demodulate;

fn print_bin(x: u8) {
    for i in 0..8 {
        if x & (1 << i) != 0 {
            print!("1");
        } else {
            print!("0");
        }
    }
}

const LOW_PASS_SOS_200K: [[f32;6];3] = [
[ 8.04235642e-07, 1.60847128e-06, 8.04235642e-07, 1.00000000e+00, -8.81618592e-01, 0.00000000e+00],
[ 1.00000000e+00, 2.00000000e+00, 1.00000000e+00, 1.00000000e+00, -1.80155740e+00, 8.15876124e-01],
[ 1.00000000e+00, 1.00000000e+00, 0.00000000e+00, 1.00000000e+00, -1.91024541e+00, 9.25427983e-01],
];

fn dump(d: &[Complex<f32>], file: &str) {
    let u8s = d.as_ptr() as *const u8;
    let u8s = unsafe { std::slice::from_raw_parts(u8s, d.len() * 8)};

    let mut out = File::create(file).unwrap();
    out.write(u8s);
}

pub fn hi() {
    let mut v = vec![];
    File::open("../s8cr8crc.dat").unwrap().read_to_end(&mut v).unwrap();
    let center = 100000.0;
    let bandwidth = 125000.0;
    let hz = 10000000.0;
    let spreading_factor = 8;

    let mut src = HackRFIQSource::new(434000000.0 - center).unwrap();

    let mut demodulator = LoraDemodulator::new(center, bandwidth, hz, spreading_factor);

    // let mut iq: Vec<_> = v.iter().tuples().map(|(i,q)|Complex::new(*i as i8 as f32, *q as i8 as f32)).collect();
    let mut iqs = vec![];
    loop {
        let mut buff = src.read();
        demodulator.feed(&buff, |_|{});
        iqs.append(&mut buff);

        if iqs.len() > 10000000 {
            dump(&iqs, "streamiq.dat");
            break;
        }
    }
    //
    // let t1 = Instant::now();
    // demodulator.feed(&iq, |packet|println!("{packet:?}"));
    // let t2 = Instant::now();
    // println!("{:?}", t2-t1);
    // return;
}
use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;
use num_complex::Complex;
use itertools::Itertools;
use rustfft::FftPlanner;
use crate::chirp::{downchirp, upchirp};
use crate::demodulate::LoraDemodulator;
use crate::gray::{from_gray, to_gray};
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
mod gray;

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

const ZEROCODE: [u8;8] = [100, 49, 28, 11, 199, 254, 178, 185];
const ONECODE: [u8;8] = [157, 80, 37, 22, 202, 251, 179, 186];

fn xoreq(a: &[u8;8], b: &[u8;8]) -> usize {
    let mut xor = [0;8];
    for i in 0..a.len(){
        xor[i] = a[i] ^ b[i];
    }
    let s = HashSet::from(xor);
    return s.len();
}

fn bitmix(x: u8, mix: &[u8;8]) -> u8 {

}

pub fn decode() {
    let mut min = 100;
    for f in [to_gray, from_gray, |x|to_gray(u8::reverse_bits(x)), |x|from_gray(u8::reverse_bits(x))] {
        for bitmix in [|x|x,|x|255-x] {
            for shift in 0..=255 {
                let zero = ZEROCODE.map(|x|f((bitmix(x)+shift)));
                let one = ONECODE.map(|x|f((bitmix(x)+shift)));
                let diffs = xoreq(&zero,&one);
                min = min.min(diffs);
                println!("{}", diffs);
            }
        }
    }
    println!("Min: {}",min);


    return;
    // let mut file = String::new();
    // File::open("packets00ff.txt").unwrap().read_to_string(&mut file).unwrap();
    //
    // file.split("\r\n").filter(|line|!line.is_empty()).for_each(|line|{
    //     let no_bracket = line.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
    //     let nums: Vec<u16> = no_bracket.split(", ")
    //         .map(|num|num.parse().unwrap())
    //         .collect();
    //
    //
    //     let nums: Vec<u16> = nums.iter().map(|&n|to_gray(n)).collect();
    //
    //     println!("{:?}",nums);
    // });

}


pub fn demod() {
    // let mut v = vec![];
    // File::open("../s8cr8crc.dat").unwrap().read_to_end(&mut v).unwrap();
    let center = 100000.0;
    let bandwidth = 125000.0;
    let hz = 10000000.0;
    let spreading_factor = 8;

    // let mut src = HackRFIQSource::new(434000000.0 - center).unwrap();
    let mut src = Box::new(HackRFIQSource::new(434000000.0 - center, hz).unwrap());

    let mut demodulator = LoraDemodulator::new(center, bandwidth, hz, spreading_factor);

    // let mut iq: Vec<_> = v.iter().tuples().map(|(i,q)|Complex::new(*i as i8 as f32, *q as i8 as f32)).collect();
    // demodulator.feed(&iq, |_|{});

    loop {
        let mut buff = src.read();
        demodulator.feed(&buff, |_|{});
    }


    //
    // let t1 = Instant::now();
    // demodulator.feed(&iq, |packet|println!("{packet:?}"));
    // let t2 = Instant::now();
    // println!("{:?}", t2-t1);
    // return;
}
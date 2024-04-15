use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};
use std::time::Instant;
use num_complex::Complex;
use itertools::Itertools;
use crate::chirp::{downchirp, upchirp};
use crate::demodulate::LoraDemodulator;
use crate::gray::{from_gray, to_gray};
use crate::hamming::{hamming48_decode, hamming48_encode};
use crate::interleaving::{deinterleave, deinterleave_block, interleave_block};
use crate::iq_source::{HackRFIQSource, IQSource};
use crate::nibble::denibble;
use crate::shifter::Shifter;
use crate::sos::sosfilt;

mod hamming;
mod shifter;
mod sos;
mod chirp;
mod iq_source;
mod demodulate;
mod gray;
mod interleaving;
mod nibble;

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

const ZERO32: [u8;88] = [33, 193, 225, 253, 161, 45, 69, 37, 100, 49, 28, 11, 199, 254, 178, 185, 195, 133, 119, 212, 223, 78, 182, 252, 117, 23, 212, 26, 178, 131, 94, 17, 42, 173, 39, 77, 130, 227, 76, 90, 100, 57, 144, 162, 227, 59, 101, 170, 8, 241, 98, 253, 7, 4, 8, 205, 4, 8, 16, 33, 196, 119, 53, 123, 53, 105, 48, 32, 70, 133, 112, 233, 89, 49, 225, 95, 69, 137, 213, 83, 206, 173, 44, 178, 77, 29, 121, 48];

fn xoreq(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), 8);
    let mut xor = [0;8];
    for i in 0..a.len(){
        xor[i] = a[i] ^ b[i];
    }
    let s = HashSet::from(xor);
    return s.len();
}

fn apply_mask(a: &[u8], b: &[u8]) -> Vec<u8> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(v1,v2)|v1^v2).collect()
}

fn degray(a: &[u8]) -> Vec<u8> {
    a.iter().map(|x|x.wrapping_sub(1)).map(to_gray).collect()
}

fn bitmix(x: u8, mix: &[u8;8]) -> u8 {
    todo!()
}

fn is_bit_set(x: u8, bit: usize) -> bool {
    (x & (1 << bit)) != 0
}

fn max_idx(arr: &[[i32;8];8]) -> (usize, usize) {
    let mut max_i = 0;
    let mut max_j = 0;
    for i in 0..8 {
        for j in 0..8 {
            if arr[i][j] > arr[max_i][max_j] {
                max_i = i;
                max_j = j;
            }
        }
    }
    return (max_i,max_j);
}

pub fn decode(packet: &[u8]) -> Vec<u8> {
    let mask = degray(&ZERO32);

    let p = degray(packet);
    let p = apply_mask(&p, &mask);
    let p = deinterleave(&p);
    let p: Vec<u8> = p.into_iter().map(hamming48_decode).collect();
    let p = denibble(&p[15..]);

    return p;
}

pub fn decode_test() {
    let mut min = 100;
    // for bitmix in [|x|x,|x|255-x,|x|to_gray(x),|x|to_gray(255-x),|x|from_gray(x),|x|from_gray(255-x)] {
    //     for shift in 0..=255 {
    //         let zero = ZEROCODE.map(bitmix);
    //     }
    // }
    // return;

    // let mask = ZEROBING2.map(|x|x.wrapping_sub(1)).map(to_gray);
    // let ones = ONEBING2.map(|x|x.wrapping_sub(1)).map(to_gray);
    // let v1 = V1.map(|x|x.wrapping_sub(1)).map(to_gray);
    //
    // for v in val.skip(16).take(8) {
    //     print!("{:#x},", v);
    // }
    // let mut mask = [0u8;40];
    // for i in 0..40 {
    //     mask[i] = zero[i] ^ ones[i];
    // }

    // println!("{:?}",mask);
    // println!("{:#x?}",&zero);
    //
    // for f in [|x|x] {
    //     for bitmix in [|x|to_gray(x),|x|to_gray(255-x)] {
    //     // for bitmix in [|x|x]{
    //         for idxshift in 8..30 {
    //             for shift in 0..=255 {
    //                 let zero = ZEROBIG.map(|x|f((bitmix(x.wrapping_add(shift)))));
    //                 let one = ONEBIG.map(|x|f((bitmix(x.wrapping_add(shift)))));
    //                 let diffs = xoreq(&zero[idxshift..(idxshift+8)],&one[idxshift..(idxshift+8)]);
    //                 if diffs == 1 || diffs == 1 {
    //                     println!("{idxshift},{shift},{}", zero[idxshift] ^ one[idxshift]);
    //                 }
    //                 min = min.min(diffs);
    //                 // println!("{}", diffs);
    //             }
    //         }
    //     }
    // }
    // println!("Min: {}",min);
    //
    //
    // return;
    let mut file = String::new();
    File::open("packetsweep4.txt").unwrap().read_to_string(&mut file).unwrap();

    let packets = file.split("\r\n").filter(|line|!line.is_empty()).map(|line|{
        let no_bracket = line.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
        let nums: Vec<u8> = no_bracket.split(", ")
            .map(|num|num.parse().unwrap())
            .collect();
        nums
    }).collect::<Vec<Vec<u8>>>();

    // let packets: Vec<_> = packets.iter().filter(|x|x[0..8] == [33,193,225,253,161,45,69,37]).collect();
    // let mut start_idx = 0;
    // let mut zero_idx = 0;
    // for i in 0..(packets.len()-9) {
    //     if packets[i] == packets[i+1] && packets[i+2] == packets[i+3] {
    //         zero_idx = i;
    //         start_idx = i + 4;
    //         println!("{zero_idx},{start_idx}");
    //     }
    // }
    // return;
    // let mut i1 = 116;
    // let mut i2 = 873;
    // for i in 0..(16*8+16+4) {
    //     if packets[i1] == packets[i2] {
    //         println!("{:?}",packets[i1]);
    //         i1 += 1;
    //         i2 += 1;
    //     } else if packets[i1+1] == packets[i2] {
    //         println!("{:?}", packets[i1]);
    //         i1 += 1;
    //     } else if packets[i1] == packets[i2+1] {
    //         println!("{:?}", packets[i2]);
    //         i2 += 1;
    //     } else {
    //         panic!("mismatch {},{}", i1, i2);
    //     }
    // }
    //
    // return;


    let zero_idx = 0;
    let start_idx = 20;
    let mask = degray(&ZERO32);

    for p in packets {
        let p = degray(&p);
        let p = apply_mask(&p, &mask);
        let p = deinterleave(&p);
        let p: Vec<u8> = p.into_iter().map(hamming48_decode).collect();
        let p = denibble(&p[15..]);
        println!("{p:?}");
    }
    //
    // for byte in 0..8 {
    //     for bit in 0..8 {
    //         let mut incomming_counts = [[0;8];8];
    //         for nibble in 0..8 {
    //             for i in 0..16 {
    //                 let packetn = &packets[nibble * 16 + i+start_idx];
    //                 let degrayed = degray(&packetn);
    //                 let xored = &apply_mask(&degrayed, &mask)[16..24];
    //
    //                 let deinter = deinterleave_block(xored.try_into().unwrap());
    //                 let decode = deinter.map(hamming48_decode);
    //                 println!("{:?}",decode);
    //             }
    //         }
    //     }
    // }


    // for i in 0..64 {
    //     let packetn = &packets[i+start_idx];
    //
    //
    //     let xored = packetn.iter()
    //         .map(|x|x.wrapping_sub(1))
    //         .map(to_gray)
    //         .zip(mask.iter())
    //         .map(|(x,mask)|x^mask)
    //     ;
    //     let ones = xored.skip(16).take(8).map(|x|x.count_ones() as usize).sum::<usize>();
    //     println!("{i} = {ones:?}");
    // }

}


pub fn demod(mut foreach: impl FnMut(Vec<u8>)) {
    // let mut v = vec![];
    // File::open("../s8cr8crc.dat").unwrap().read_to_end(&mut v).unwrap();
    let center = 100000.0;
    let bandwidth = 125000.0;
    let hz = 10000000.0;
    let spreading_factor = 8;

    // let mut src = HackRFIQSource::new(434000000.0 - center).unwrap();
    let mut src = HackRFIQSource::new(434000000.0 - center, hz).unwrap();

    let mut demodulator = LoraDemodulator::new(center, bandwidth, hz, spreading_factor);

    // let mut iq: Vec<_> = v.iter().tuples().map(|(i,q)|Complex::new(*i as i8 as f32, *q as i8 as f32)).collect();
    // demodulator.feed(&iq, |_|{});

    loop {
        let mut buff = src.read();
        demodulator.feed(&buff, &mut foreach);
    }
}
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use crate::chirp::{downchirp, upchirp};
use crate::crcs::check;
use crate::demodulate::LoraDemodulator;
use crate::gray::to_gray;
use crate::hamming::{hamming4_n_decode, hamming4_n_encode};
use crate::interleaving::deinterleave;
use crate::iq_source::{HackRFIQSource, IQSource};
use crate::nibble::denibble;
use crate::shifter::Shifter;
use crate::sos::sosfilt;
use crate::symbol::{Packet, Symbol};

mod hamming;
mod shifter;
mod sos;
mod chirp;
mod iq_source;
mod demodulate;
mod gray;
mod interleaving;
mod nibble;
mod crcs;
mod symbol;

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

const ZERO_CR8: [u8;88] = [33, 193, 225, 253, 161, 45, 69, 37, 100, 49, 28, 11, 199, 254, 178, 185, 195, 133, 119, 212, 223, 78, 182, 252, 117, 23, 212, 26, 178, 131, 94, 17, 42, 173, 39, 77, 130, 227, 76, 90, 100, 57, 144, 162, 227, 59, 101, 170, 8, 241, 98, 253, 7, 4, 8, 205, 4, 8, 16, 33, 196, 119, 53, 123, 53, 105, 48, 32, 70, 133, 112, 233, 89, 49, 225, 95, 69, 137, 213, 83, 206, 173, 44, 178, 77, 29, 121, 48];
const ZERO_CR7: [u8;74] = [93, 205, 229, 1, 165, 17, 57, 25, 100, 49, 28, 11, 199, 254, 178, 195, 133, 119, 212, 223, 78, 182, 117, 23, 212, 26, 178, 131, 94, 42, 173, 39, 77, 130, 227, 76, 100, 57, 144, 162, 227, 59, 101, 8, 241, 98, 253, 7, 4, 8, 4, 8, 16, 33, 196, 119, 53, 53, 105, 48, 32, 70, 133, 112, 89, 49, 225, 95, 69, 137, 213, 206, 173, 44];
const ZERO_CR6: [u8;66] = [33, 253, 249, 1, 221, 45, 37, 25, 100, 49, 28, 11, 199, 254, 195, 133, 119, 212, 223, 78, 117, 23, 212, 26, 178, 131, 42, 173, 39, 77, 130, 227, 100, 57, 144, 162, 227, 59, 8, 241, 98, 253, 7, 4, 4, 8, 16, 33, 196, 119, 53, 105, 48, 32, 70, 133, 89, 49, 225, 95, 69, 137, 206, 173, 44, 178];
const ZERO_CR5: [u8;58] = [33, 241, 225, 1, 217, 209, 57, 41, 100, 49, 28, 11, 61, 195, 133, 119, 212, 184, 117, 23, 212, 26, 67, 42, 173, 39, 77, 168, 100, 57, 144, 162, 78, 8, 241, 98, 253, 121, 4, 8, 16, 33, 212, 53, 105, 48, 32, 75, 89, 49, 225, 95, 149, 206, 173, 44, 178, 236];

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

fn max_idx<const N: usize, const M: usize>(arr: &[[i32;N];M]) -> (usize, usize) {
    let mut max_i = 0;
    let mut max_j = 0;
    for i in 0..M {
        for j in 0..N {
            if arr[i][j] > arr[max_i][max_j] {
                max_i = i;
                max_j = j;
            }
        }
    }
    return (max_i,max_j);
}

pub fn decode(packet: &Packet) -> Vec<u8> {
    let mask = degray(&ZERO_CR7);

    let u8s = packet.symbols.iter().map(|s|s.value as u8).collect::<Vec<_>>();
    let p = degray(&u8s);
    let p = apply_mask(&p, &mask);
    let p = &p[8..]; //skip header
    let p = deinterleave::<7>(p);
    let p: Vec<u8> = p.into_iter().map(hamming4_n_decode::<7>).collect();
    let p = denibble(&p[7..]);//first 7 are unknown

    return p;
}

fn read_packet_file(path: &str) -> Vec<Vec<u8>>{
    let mut file = String::new();
    File::open(path).unwrap().read_to_string(&mut file).unwrap();

    let packets = file.split("\r\n").filter(|line|!line.is_empty()).map(|line|{
        let no_bracket = line.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
        let nums: Vec<u8> = no_bracket.split(", ")
            .map(|num|num.parse().unwrap())
            .collect();
        nums
    }).collect::<Vec<Vec<u8>>>();

    packets
}

pub fn crc() {
    let mut file = String::new();
    File::open("crc.txt").unwrap().read_to_string(&mut file).unwrap();

    let packets = file.split("\r\n").filter(|line|!line.is_empty()).map(|line|{
        let no_bracket = line.strip_prefix('[').unwrap().strip_suffix(']').unwrap();
        let nums: Vec<u8> = no_bracket.split(", ")
            .map(|num|num.parse().unwrap())
            .collect();
        nums
    }).collect::<Vec<Vec<u8>>>();

    check(&packets);
}

//packets alerady degrayed and xored
fn deduce_interleave<const CR: usize>(interleaved: &[Vec<u8>]) -> Vec<(usize,usize)> {
    let mut out = vec![];
    for byte in 0..CR {
        for bit in 0..8 {
            let mut counts = [[0;CR];8];
            for nibble in 0..8 {
                for i in 0..16 {
                    let packet_bit = is_bit_set(interleaved[nibble*16+i][byte], bit);
                    let ham = hamming4_n_encode::<CR>(i as u8);
                    for bit in 0..CR {
                        if packet_bit && is_bit_set(ham,bit) {
                            counts[nibble][bit] += 1;
                        }
                    }
                }
            }
            println!("{:?}",counts);
            out.push(max_idx(&counts));
        }
    }

    out
}

pub fn decode_header() {
    let packets = read_packet_file("packets47_1.txt");

    let start = 303+16;

    let zero = degray(&packets[start]);
    let one = degray(&packets[start+1]);

    let inter: Vec<Vec<u8>> = (0..(16*8)).map(|i|&packets[start+i]).map(|p|{
        let p = degray(&p);
        let p = apply_mask(&zero,&p);
        p[15..22].to_vec()
    }).collect();

    println!("{:?}",inter);
    let interleave = deduce_interleave::<7>(&inter);
    println!("{:?}", interleave);

    // for i in 0..32 {
    //     let packetn = &packets[i+start];
    //     let p = packetn.iter().take(8).map(|x|x.wrapping_sub(1)).map(|x|x/4).map(to_gray).collect::<Vec<u8>>();
    //
    //     let p = apply_mask(&p, &zero);
    //     print!("{:3}= ",i);
    //     p.iter().for_each(|x|{print_bin(*x);print!(" ");});
    //     println!();
    //     // let ones = p.iter().map(|x|x.count_ones()).sum::<u32>();
    //     // println!("{} = {}", i, ones);
    // }
}

pub fn demod(mut foreach: impl FnMut(Packet)) {
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
        let buff = src.read();
        demodulator.feed(&buff, &mut foreach);
    }
}
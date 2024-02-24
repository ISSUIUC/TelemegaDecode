mod shifter;
mod decoder;
mod streaming_gfsk;
mod transition;
mod streaming_bit_sync;
mod streaming_fec_decoder;
mod ao;
mod iq_source;

use std::collections::VecDeque;
use std::io::{stdout, Write};
use clap::Parser;
use num_complex::Complex;
use crate::iq_source::HackRFIQSource;
use crate::streaming_bit_sync::StreamingBitSync;
use crate::streaming_fec_decoder::{Packet, StreamingFecDecoder};
use crate::streaming_gfsk::StreamingGFSKDecoder;

const HZ: f64 = 20_000_000.0;
const BAUD: f64 = 38400.0;


#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    frequencies: Vec<f64>
}

impl Arguments {
    fn get_center(&self) -> f64 {
        self.frequencies.iter()
            .max_by(|a, b| a.total_cmp(b))
            .map_or(0.0, |max| max + 100_000.0)
    }
}


struct Decoder {
    ct: usize,

    fec: StreamingFecDecoder,
    bit: StreamingBitSync,
    gfsk: StreamingGFSKDecoder,

    packets: VecDeque<Packet>
}

impl Decoder {
    fn new(center: f64, hz: f64, baud: f64) -> Decoder {
        Decoder {
            ct: 0,
            fec: StreamingFecDecoder::new(),
            bit: StreamingBitSync::new(hz/baud),
            gfsk: StreamingGFSKDecoder::new(hz, center),
            packets: VecDeque::with_capacity(1000)
        }
    }

    fn feed(&mut self, value: Complex<f64>) {
        self.ct += 1;
        if self.ct >= 20_000_000 {
            self.ct = 0;
            println!("Got 20M");
        }

        self.gfsk.feed(value, |trans| {
            self.bit.feed(trans, |bit| {
                self.fec.feed(bit, |packet| {
                    self.packets.push_back(packet);
                });
            });
        });
    }
}


fn main() {
    let args = Arguments::parse();
    if args.frequencies.is_empty() {
        panic!("Requires at least one frequency argument");
    }

    let center = args.get_center();

    let mut decoders: Vec<Decoder> = args.frequencies.iter().map(|freq| {
        Decoder::new(freq - center, HZ, BAUD)
    }).collect();

    let (tx, rx) = std::sync::mpsc::sync_channel(100);
    let recv_handle = std::thread::spawn(move || {
        println!("Constructing HackRFIQSource...");
        let mut src = HackRFIQSource::new(center);
        println!("Constructed HackRFIQSource!");
        loop {
            let buffer = src.read();
            if buffer.is_empty() {
                return;
            }
            tx.send(buffer).unwrap();
        }
    });

    for buffer in rx {
        for item in buffer {
            for decoder in &mut decoders {
                decoder.feed(item);
                while let Some(packet) = decoder.packets.pop_front() {
                    println!("packet: {:?}", packet);
                }
            }
        }
    }

    recv_handle.join().unwrap();
}
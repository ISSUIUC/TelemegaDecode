mod shifter;
mod decoder;
mod streaming_gfsk;
mod transition;
mod streaming_bit_sync;
mod streaming_fec_decoder;
mod ao;
mod iq_source;
mod types;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{Write};
use std::path::PathBuf;
use std::thread::JoinHandle;
use std::time::Instant;
use clap::Parser;
use num_complex::Complex;
use crate::iq_source::{FileIQSource, HackRFIQSource, IQSource};
use crate::streaming_bit_sync::StreamingBitSync;
use crate::streaming_fec_decoder::{Packet, StreamingFecDecoder};
use crate::streaming_gfsk::StreamingGFSKDecoder;

const HZ: f64 = 20_000_000.0;
const BAUD: f64 = 38400.0;


#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    frequencies: Vec<f64>,
    #[arg(short, long)]
    file: Option<PathBuf>
}

impl Arguments {
    fn get_center(&self) -> f64 {
        self.frequencies.iter()
            .max_by(|a, b| a.total_cmp(b))
            .map_or(0.0, |max| max + 100_000.0)
    }
}


struct Decoder {
    // file: File,

    ct: usize,

    fec: StreamingFecDecoder,
    bit: StreamingBitSync,
    gfsk: StreamingGFSKDecoder,

    packets: VecDeque<Packet>
}

impl Decoder {
    fn new(center: f64, hz: f64, baud: f64) -> Decoder {
        Decoder {
            // file: File::create("log.txt").unwrap(),

            ct: 0,
            fec: StreamingFecDecoder::new(),
            bit: StreamingBitSync::new(hz/baud),
            gfsk: StreamingGFSKDecoder::new(hz, center),
            packets: VecDeque::with_capacity(1000)
        }
    }

    fn feed(&mut self, value: &[Complex<f32>]) {
        self.ct += value.len();
        if self.ct >= 20_000_000 {
            self.ct = 0;
            println!("Got 20M");
        }

        self.gfsk.feed(value, |trans| {
            self.bit.feed(trans, |bit| {
                // self.file.write(&[bit as u8]).unwrap();
                // write!(self.file, "{}", if bit { '1' } else { '0' }).unwrap();
                self.fec.feed(bit, |packet| {
                    self.packets.push_back(packet);
                });
            });
        });
    }
}


fn spawn_reading_thread<S: IQSource + Send + 'static>(mut src: S) -> (JoinHandle<()>, std::sync::mpsc::Receiver<Vec<Complex<f32>>>) {
    let (tx, rx) = std::sync::mpsc::channel();
    let recv_handle = std::thread::spawn(move || {
        loop {
            let buffer = src.read();
            if buffer.is_empty() {
                return;
            }
            tx.send(buffer).unwrap();
        }
    });

    (recv_handle, rx)
}


fn main() {
    let start = Instant::now();
    let args = Arguments::parse();
    if args.frequencies.is_empty() {
        panic!("Requires at least one frequency argument");
    }

    let center = args.get_center();

    let mut decoders: Vec<Decoder> = args.frequencies.iter().map(|freq| {
        Decoder::new(freq - center, HZ, BAUD)
    }).collect();

    let (recv_handle, rx) = if let Some(file_name) = args.file {
        spawn_reading_thread(FileIQSource::new(file_name))
    } else {
        spawn_reading_thread(HackRFIQSource::new(args.frequencies[0]))
    };

    for buffer in rx {
        decoders[0].feed(&buffer);
        while let Some(packet) = decoders[0].packets.pop_front() {
            println!("packet: {:?}", packet);
        }
        // for decoder in &mut decoders {
        //     decoder.feed(&buffer);
        //     while let Some(packet) = decoder.packets.pop_front() {
        //         println!("packet: {:?}", packet);
        //     }
        // }
    }

    let end = Instant::now();
    println!("Took {:?} sec", (end - start));

    recv_handle.join().unwrap();
}
mod shifter;
mod decoder;
mod streaming_gfsk;
mod transition;
mod streaming_bit_sync;
mod streaming_fec_decoder;
mod ao;
mod iq_source;

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use clap::Parser;
use num_complex::Complex;
use bus::Bus;
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
    fec: StreamingFecDecoder,
    bit: StreamingBitSync,
    gfsk: StreamingGFSKDecoder,

    packets: VecDeque<Packet>
}

impl Decoder {
    fn new(center: f64, hz: f64, baud: f64) -> Decoder {
        Decoder {
            fec: StreamingFecDecoder::new(),
            bit: StreamingBitSync::new(hz/baud),
            gfsk: StreamingGFSKDecoder::new(hz, center),
            packets: VecDeque::with_capacity(1000)
        }
    }

    fn feed(&mut self, value: &[Complex<f32>]) {
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
    let start = Instant::now();
    let args = Arguments::parse();
    if args.frequencies.is_empty() {
        panic!("Requires at least one frequency argument");
    }

    let center = args.get_center();

    let mut src: Box<dyn IQSource + Send> = if let Some(file_name) = args.file {
        Box::new(FileIQSource::new(file_name))
    } else {
        Box::new(HackRFIQSource::new(center))
    };

    let mut bus: Bus<Arc<Vec<Complex<f32>>>> = Bus::new(10000);

    let mut worker_handles = Vec::new();
    for freq in args.frequencies {
        let mut decoder = Decoder::new(freq - center, HZ, BAUD);
        let mut bus_recv = bus.add_rx();
        let handle = std::thread::spawn(move || {
            while let Ok(packet) = bus_recv.recv() {
                decoder.feed(&packet);
                while let Some(packet) = decoder.packets.pop_front() {
                    println!("{:?}", packet);
                }
            }
        });
        worker_handles.push(handle);
    }

    std::thread::spawn(move || {
        loop {
            let buffer = src.read();
            if buffer.is_empty() {
                return;
            }
            bus.broadcast(Arc::new(buffer));
        }
    });

    worker_handles.into_iter().for_each(|handle| handle.join().unwrap());

    let end = Instant::now();
    println!("Took {:?}", end - start);
}
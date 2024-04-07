mod shifter;
mod streaming_gfsk;
mod transition;
mod streaming_bit_sync;
mod streaming_fec_decoder;
mod ao;
mod iq_source;
mod packet;
mod packet_types;

use std::collections::VecDeque;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use clap::Parser;
use num_complex::Complex;
use bus::Bus;
use crate::iq_source::{FileIQSource, HackRFIQSource, IQSource};
use crate::packet::Packet;
use crate::packet_types::decode;
use crate::streaming_bit_sync::StreamingBitSync;
use crate::streaming_fec_decoder::StreamingFecDecoder;
use crate::streaming_gfsk::StreamingGFSKDecoder;

pub use crate::packet_types::*;

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

struct FullDecoder {
    fec: StreamingFecDecoder,
    bit: StreamingBitSync,
    gfsk: StreamingGFSKDecoder,
    packets: VecDeque<Packet>,
    center: f64,
}

impl FullDecoder {
    fn new(center: f64, hz: f64, baud: f64) -> FullDecoder {
        FullDecoder {
            fec: StreamingFecDecoder::new(),
            bit: StreamingBitSync::new(hz/baud),
            gfsk: StreamingGFSKDecoder::new(hz, center),
            packets: VecDeque::with_capacity(1000),
            center
        }
    }

    fn get_queued(&mut self) -> Option<Packet> {
        self.packets.pop_front()
    }

    fn center(&self) -> f64 {
        self.center
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

pub fn start_decoders(new_packet: impl Fn(f64, DecodedPacket) + Sync) {
    let start = Instant::now();
    let args = Arguments::parse();
    if args.frequencies.is_empty() {
        panic!("Requires at least one frequency argument");
    }

    let center = args.get_center();
    let mut src: Box<dyn IQSource + Send> = if let Some(file_name) = args.file {
        Box::new(FileIQSource::new(file_name))
    } else {
        Box::new(HackRFIQSource::new(center).unwrap())
    };

    let mut bus: Bus<Arc<Vec<Complex<f32>>>> = Bus::new(10000);


    std::thread::scope(|scope| {
        let mut worker_handles = Vec::new();
        for freq in args.frequencies {
            let callback_ref = &new_packet;
            let mut bus_recv = bus.add_rx();
            let handle = scope.spawn(move || {
                let mut decoder = FullDecoder::new(freq - center, HZ, BAUD);
                while let Ok(packet) = bus_recv.recv() {
                    decoder.feed(&packet);
                    while let Some(packet) = decoder.get_queued() {
                        if let Ok(decoded) = decode(&packet) {
                            callback_ref(decoder.center(), decoded);
                        }
                    }
                }
            });
            worker_handles.push(handle);
        }

        scope.spawn(move || {
            let mut log = std::fs::File::create("../../loggy.dat").unwrap();
            loop {
                let buffer = src.read();
                let mut u8 = Vec::new();
                for c in &buffer {
                    u8.push(c.im as u8);
                    u8.push(c.re as u8);
                }
                let _ = log.write(&u8);
                if buffer.is_empty() {
                    return;
                }
                bus.broadcast(Arc::new(buffer));
            }
        });

        worker_handles.into_iter().for_each(|handle| handle.join().unwrap());
    });

    let end = Instant::now();
    println!("Took {:?}", end - start);
}

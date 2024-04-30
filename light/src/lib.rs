use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::Arc;
use std::time::Instant;
use itertools::Itertools;
use num_complex::{Complex, Complex32, ComplexFloat};
use rustfft::{Fft, FftDirection, FftPlanner};
use rustfft::num_traits::Signed;
use crate::iq_source::{FileIQSource, HackRFIQSource, IQSource};
use crate::shifter::Shifter;
use crate::sos::sosfilt;

mod iq_source;
mod shifter;
mod sos;

const SOS_500K: [[f32;6];3] = [
    [ 2.34099149e-06,4.68198299e-06,2.34099149e-06,1.00000000e+00,-8.54080685e-01,0.00000000e+00],
    [ 1.00000000e+00,2.00000000e+00,1.00000000e+00,1.00000000e+00,-1.75346181e+00,7.75318936e-01],
    [ 1.00000000e+00,1.00000000e+00,0.00000000e+00,1.00000000e+00,-1.88428847e+00,9.07776358e-01]
];

fn conj_mult(a: &[Complex32], b: &[Complex32]) -> Vec<Complex32> {
    assert_eq!(a.len(), b.len());
    let mut ret = vec![];
    ret.resize(a.len(), Complex::new(0.0,0.0));

    for i in 0..a.len() {
        ret[i] = a[i] * b[i].conj();
    }

    return ret;
}

fn dump(data: &[Complex32]) {
    let u8 = data.as_ptr() as *const u8;
    let u8 = unsafe {std::slice::from_raw_parts(u8, data.len() * 8)};
    let mut f = std::fs::File::create("dat.dat").unwrap();
    f.write(u8).unwrap();
}

struct LightDelayer {
    w: usize,
    forward: Arc<dyn Fft<f32>>,
    backward: Arc<dyn Fft<f32>>,
    shift_ping: Shifter,
    shift_pong: Shifter,
    delay: usize,
}

impl LightDelayer {
    fn new(w: usize, hz: f64, pingoff: f64, pongoff: f64, delay: usize) -> Self {
        let mut planner = FftPlanner::new();
        let forward = planner.plan_fft(w, FftDirection::Forward);
        let backward = planner.plan_fft(w, FftDirection::Inverse);
        let shift_ping = Shifter::new(pingoff, hz);
        let shift_pong = Shifter::new(pongoff, hz);

        Self {
            w,
            forward,
            backward,
            shift_pong,
            shift_ping,
            delay,
        }
    }

    fn process(&self, chunk: Vec<Complex32>) -> Vec<f64> {
        let saved_chunj = chunk.clone();
        let mut ping = chunk.clone();
        let mut pong = chunk;

        self.shift_ping.shift(&mut ping, 0);
        self.shift_pong.shift(&mut pong, 0);

        let mut zi0 = [[Complex::new(0.0,0.0);2];3];
        let mut zi1 = [[Complex::new(0.0,0.0);2];3];
        sosfilt(&SOS_500K, &mut ping, &mut zi0);
        sosfilt(&SOS_500K, &mut pong, &mut zi1);

        let len = ping.len();
        let ping = &mut ping[0..(len-self.delay)];
        let pong = &mut pong[self.delay..];

        let mut delays = vec![];
        let w = self.w;
        for i in 0..(ping.len()/w) {
            let ping = &mut ping[(i*w)..(i*w+w)];
            let pong = &mut pong[(i*w)..(i*w+w)];

            self.forward.process(ping);
            self.forward.process(pong);

            let mut corr = conj_mult(&ping,&pong);
            self.backward.process(&mut corr);

            let idx = corr.iter().map(|x|x.abs()).position_max_by(|a,b|a.partial_cmp(b).unwrap()).unwrap();
            let c = corr[idx].abs();
            let l = corr[idx-1].abs();
            let r = corr[idx+1].abs();

            let ldiff = c - l;
            let rdiff = c - r;
            let adj = ldiff / (rdiff + ldiff) - 0.5;

            delays.push(idx as f64 + adj as f64);
            // if idx > 16000 {
            //     println!("{:?}", delays);
            //     println!("{},{}", idx, i);
            //     dump(&saved_chunj);
            //     panic!();
            // }
        }

        // println!("{}",delays.iter().sum::<usize>() as f64 / delays.len() as f64);
        // println!("{:?}",delays);
        return delays;
    }
}

pub fn hi() {
    let hz = 20000000.0;
    let center = 435500000.0;
    let center_ping = 433000000.0;
    let center_pong = 435000000.0;

    let mut f = File::open("delay.dat").unwrap();
    let mut v = vec![];
    f.read_to_end(&mut v).unwrap();
    let delay = 350000;

    let mut src = HackRFIQSource::new(435500000.0, hz).unwrap();
    let calc = Arc::new(LightDelayer::new(1250000, hz, center - center_ping,center - center_pong,delay));
    let mut chunk = vec![];
    let mut inflight = vec![];

    let mut f = File::create("nums6.txt").unwrap();
    loop {
        let mut buff = src.read();
        chunk.append(&mut buff);
        if chunk.len() >= 20000000 {
            let mut c = vec![];
            std::mem::swap(&mut chunk, &mut c);
            let delayer = calc.clone();
            let task = std::thread::spawn(move ||{delayer.process(c)});
            inflight.push(task);
        }
        if !inflight.is_empty() && inflight[0].is_finished() {
            let task = inflight.remove(0);
            let res = task.join().unwrap();
            // let res = filter(&res);
            print!("{res:?}");
            println!(" {}", res.iter().sum::<f64>() as f64 / res.len() as f64);
            for i in res {
                write!(f,"{i} ");
            }
        }
    }
}
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use itertools::Itertools;
use hackrfone::{HackRfOne, RxMode};
use num_complex::Complex;

trait IQSource : Iterator<Item=Complex<f64>> { }
impl<I> IQSource for I where I: Iterator<Item=Complex<f64>> { }


const SAMPLE_RATE: f64 = 20_000_000.0;
const MAX_GAIN_SETTING: i16 = 36;
const MIN_GAIN_SETTING: i16 = 0;


pub struct HackRFIQSource {
    current_gain: i16,
    max_iq_reading: i8,
    amp_adjust_time: i64,
    backlog: VecDeque<Vec<Complex<f64>>>,

    hack_rf: HackRfOne<RxMode>,
    _not_send: *const u8
}

extern "C" { fn hackrf_get_sample_rate(freq: f64, freq_hz: *mut u32, divider: *mut u32); }

fn gain_index(idx: i16) -> (i16, i16) {
    if idx < 0 {
        (0, 0)
    } else if idx <= 5 {
        (idx * 8, 0)
    } else if idx <= 5 + 31 {
        (40, (idx - 5) * 2)
    } else {
        (40, 62)
    }
}

impl HackRFIQSource {
    pub fn new(center: f64) -> HackRFIQSource {
        let mut hack_rf = HackRfOne::new().unwrap();

        let mut freq_hz = 0;
        let mut divider = 0;
        unsafe {
            hackrf_get_sample_rate(SAMPLE_RATE, &mut freq_hz, &mut divider);
        }
        hack_rf.set_sample_rate(freq_hz, divider).unwrap();
        hack_rf.set_freq(center as u64).unwrap();
        hack_rf.set_amp_enable(true).unwrap();
        let (lna, vga) = gain_index(6);
        hack_rf.set_vga_gain(vga as u16).unwrap();
        hack_rf.set_lna_gain(lna as u16).unwrap();
        let hack_rf = hack_rf.into_rx_mode().unwrap();

        HackRFIQSource {
            current_gain: 6,
            max_iq_reading: 0,
            amp_adjust_time: SAMPLE_RATE as i64,
            backlog: VecDeque::with_capacity(1000),
            hack_rf,
            _not_send: std::ptr::null()
        }
    }

    pub fn read(&mut self) -> Vec<Complex<f64>> {
        let buffer = self.hack_rf.rx().unwrap();
        let mut out = Vec::with_capacity(buffer.len() / 2);
        for (i, q) in buffer.into_iter().tuples() {
            out.push(Complex::new((i as i8) as f64, (q as i8) as f64));
        }

        self.amp_adjust_time -= out.len() as i64;
        if self.amp_adjust_time <= 0 {
            let changed;
            let new_gain;
            if self.max_iq_reading < 40 {
                new_gain = i16::max(self.current_gain + 1, MAX_GAIN_SETTING);
                changed = true;
            } else if self.max_iq_reading > 90 {
                new_gain = i16::max(self.current_gain - 1, MIN_GAIN_SETTING);
                changed = true;
            } else {
                new_gain = 0;
                changed = false;
            }

            if changed {
                let (lna, vga) = gain_index(new_gain);
                unsafe {
                    let mut hack_rf = std::ptr::read(&self.hack_rf).stop_rx().unwrap();
                    hack_rf.set_vga_gain(vga as u16).unwrap();
                    hack_rf.set_lna_gain(lna as u16).unwrap();
                    std::ptr::write(&mut self.hack_rf, hack_rf.into_rx_mode().unwrap());
                }
                self.current_gain = new_gain;
            }
            self.max_iq_reading = 0;
            self.amp_adjust_time = SAMPLE_RATE as i64;
        }

        out
    }
}
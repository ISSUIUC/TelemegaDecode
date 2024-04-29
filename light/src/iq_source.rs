use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use itertools::Itertools;
use async_libhackrf::{HackRfOne, RxMode};
use num_complex::Complex;


pub trait IQSource {
    fn read(&mut self) -> Vec<Complex<f32>>;
}


const MAX_GAIN_SETTING: i16 = 36;
const MIN_GAIN_SETTING: i16 = 0;


pub struct FileIQSource {
    file: BufReader<File>
}


impl FileIQSource {
    pub fn new(file_name: impl AsRef<Path>) -> FileIQSource {
        FileIQSource {
            file: BufReader::new(File::open(file_name.as_ref()).unwrap())
        }
    }
}

const BUF_SIZE: usize = 1024 * 1024;

impl IQSource for FileIQSource {
    fn read(&mut self) -> Vec<Complex<f32>> {
        let mut buf = [0; BUF_SIZE];
        match self.file.read(&mut buf) {
            Ok(ct) => {
                let mut out = Vec::with_capacity(ct/2);
                for i in (0..ct).step_by(2) {
                    out.push(Complex::new(buf[i] as i8 as f32, buf[i+1] as i8 as f32))
                }
                out
            }
            Err(_) => vec![]
        }
    }
}

pub struct HackRFIQSource {
    current_gain: i16,
    max_iq_reading: i8,
    amp_adjust_time: i64,
    hack_rf: HackRfOne<RxMode>,
    sample_rate: f64,
}

extern "C" { fn hackrf_get_sample_rate(freq: f64, freq_hz: *mut u32, divider: *mut u32); }

fn gain_index(idx: i16) -> (i16, i16) {
    return (40,0);
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
    pub fn new(center: f64, sample_rate: f64) -> Result<HackRFIQSource, async_libhackrf::Error> {
        let mut hack_rf = HackRfOne::new().unwrap();

        let mut freq_hz = 0;
        let mut divider = 0;
        unsafe {
            hackrf_get_sample_rate(sample_rate, &mut freq_hz, &mut divider);
        }
        hack_rf.set_sample_rate(freq_hz, divider)?;
        hack_rf.set_freq(center as u64)?;
        hack_rf.set_amp_enable(true)?;
        let (lna, vga) = gain_index(6);
        hack_rf.set_vga_gain(vga as u16)?;
        hack_rf.set_lna_gain(lna as u16)?;
        let hack_rf = hack_rf.into_rx_mode().unwrap();

        Ok(HackRFIQSource {
            current_gain: 6,
            max_iq_reading: 0,
            amp_adjust_time: sample_rate as i64,
            sample_rate,
            hack_rf
        })
    }
}

impl IQSource for HackRFIQSource {
    fn read(&mut self) -> Vec<Complex<f32>> {
        let buffer = self.hack_rf.rx().unwrap();
        let mut out = Vec::with_capacity(buffer.len() / 2);
        for (i, q) in buffer.into_iter().tuples() {
            self.max_iq_reading = i8::max(self.max_iq_reading, i as i8);
            self.max_iq_reading = i8::max(self.max_iq_reading, q as i8);
            out.push(Complex::new((i as i8) as f32, (q as i8) as f32));
        }

        self.amp_adjust_time -= out.len() as i64;
        if self.amp_adjust_time <= 0 && false {
            let changed;
            let new_gain;
            if self.max_iq_reading < 40 {
                new_gain = i16::min(self.current_gain + 1, MAX_GAIN_SETTING);
                changed = true;
            } else if self.max_iq_reading > 90 {
                new_gain = i16::max(self.current_gain - 1, MIN_GAIN_SETTING);
                changed = true;
            } else {
                new_gain = 0;
                changed = false;
            }

            if changed {
                // eprintln!("Oh no we couldn't change stuff");
                let (lna, vga) = gain_index(new_gain);
                unsafe {
                    // the unsafe is a workaround so that we can move out to the hack_rf
                    // we need to move out to call stop_rx
                    let mut hack_rf = std::ptr::read(&self.hack_rf).stop_rx().unwrap();
                    hack_rf.set_vga_gain(vga as u16).unwrap();
                    hack_rf.set_lna_gain(lna as u16).unwrap();
                    println!("gain = {vga},{lna} max = {}", self.max_iq_reading);
                    std::ptr::write(&mut self.hack_rf, hack_rf.into_rx_mode().unwrap());
                }
                self.current_gain = new_gain;
            }
            self.max_iq_reading = 0;
            self.amp_adjust_time = self.sample_rate as i64;
        }

        out
    }
}
use std::sync::Arc;
use num_complex::{Complex, Complex32};
use rustfft::{Fft, FftPlanner};
use crate::{downchirp, Shifter, sosfilt, upchirp};
use crate::symbol::{Packet, Symbol};

const BUFFER_SIZE: usize = 1024 * 32;

enum SyncingMode {
    LookSync,
    Look8,
    Look16,
    LookDown0,
    LookDown1,
    LookDownQuarter,
    Synced,
}

enum ChirpKind {
    Up,
    Down,
}

fn product(a: &mut[Complex<f32>], b: &[Complex<f32>]) {
    assert_eq!(a.len(), b.len());

    for i in 0..a.len() {
        a[i] *= b[i]
    }
}

fn mod_distance(a: usize, b: usize, mod_base: usize) -> usize {
    let d = usize::max(a,b) - usize::min(a,b) % mod_base;
    return usize::min(d, mod_base - d);
}

fn all_within(symbols: &[Symbol], range: usize, mod_base: usize) -> bool {
    if symbols.is_empty() {
        return true;
    }
    for i in 0..mod_base {
        if symbols.iter().map(|x| mod_distance(x.value,i,mod_base)).max().unwrap() <= range {
            return true;
        }
    }

    return false;
}

const LOW_PASS_SOS_200K: [[f32;6];3] = [
    [ 8.04235642e-07, 1.60847128e-06, 8.04235642e-07, 1.00000000e+00, -8.81618592e-01, 0.00000000e+00],
    [ 1.00000000e+00, 2.00000000e+00, 1.00000000e+00, 1.00000000e+00, -1.80155740e+00, 8.15876124e-01],
    [ 1.00000000e+00, 1.00000000e+00, 0.00000000e+00, 1.00000000e+00, -1.91024541e+00, 9.25427983e-01],
];

struct LoraSyncStateMachine {
    sync_state: SyncingMode,
    up_history: Vec<Symbol>,
    down_history: Vec<Symbol>,
    packet: Vec<Symbol>,
    sf2: usize,
}

impl LoraSyncStateMachine {
    fn new(spreading_factor: u32) -> Self {
        Self {
            sync_state: SyncingMode::LookSync,
            up_history: vec![],
            down_history: vec![],
            packet: vec![],
            sf2: 2_u32.pow(spreading_factor) as usize
        }
    }

    fn which_chirp(&self) -> ChirpKind {
        match self.sync_state {
            SyncingMode::LookSync => ChirpKind::Up,
            SyncingMode::Look8 => ChirpKind::Up,
            SyncingMode::Look16 => ChirpKind::Up,
            SyncingMode::LookDown0 => ChirpKind::Down,
            SyncingMode::LookDown1 => ChirpKind::Down,
            SyncingMode::LookDownQuarter => ChirpKind::Down,
            SyncingMode::Synced => ChirpKind::Up,
        }
    }

    fn next(&mut self, sym: Symbol, mut foreach: impl FnMut(Packet)) -> f64 {
        match self.sync_state {
            SyncingMode::LookSync => self.up_history.push(sym),
            SyncingMode::Look8 => self.up_history.push(sym),
            SyncingMode::Look16 => self.up_history.push(sym),
            SyncingMode::LookDown0 => self.down_history.push(sym),
            SyncingMode::LookDown1 => self.down_history.push(sym),
            SyncingMode::LookDownQuarter => self.down_history.push(sym),
            SyncingMode::Synced => self.up_history.push(sym),
        }

        let mut shift_amount = 0.0;

        self.sync_state = match self.sync_state {
            SyncingMode::LookSync => {
                if self.up_history.len() > 7 && all_within(&self.up_history[(self.up_history.len()-7)..self.up_history.len()], 2, self.sf2) {
                    shift_amount = (sym.value as f64 + sym.adj as f64)/(self.sf2 as f64);
                    SyncingMode::Look8
                } else {
                    SyncingMode::LookSync
                }
            }
            SyncingMode::Look8 => {
                match sym.value {
                    0 => SyncingMode::Look8,
                    8 => SyncingMode::Look16,
                    _ => {SyncingMode::LookSync},
                }
            }
            SyncingMode::Look16 => {
                match sym.value {
                    16  => SyncingMode::LookDown0,
                    _ => {eprintln!("fail16"); SyncingMode::LookSync},
                }
            }
            SyncingMode::LookDown0 => {
                SyncingMode::LookDown1
            }
            SyncingMode::LookDown1 => {
                if mod_distance(self.down_history[self.down_history.len()-1].value, self.down_history[self.down_history.len()-2].value, self.sf2) <= 1 {
                    shift_amount = 0.75;
                    SyncingMode::LookDownQuarter
                } else {
                    eprintln!("faildown");
                    SyncingMode::LookSync
                }
            }
            SyncingMode::LookDownQuarter => {
                SyncingMode::Synced
            }
            SyncingMode::Synced => {
                self.packet.push(sym);
                if self.packet.len() > 73 {
                    foreach(Packet::new(self.packet.clone()));
                    self.packet.clear();
                    SyncingMode::LookSync
                } else {
                    SyncingMode::Synced
                }
            }
        };

        shift_amount.max(0.0)
    }
}

pub struct LoraDemodulator {
    shifter: Shifter,
    center: f64,
    hz: f64,
    bandwidth: f64,
    sf: u32,
    sf2: usize,
    zi: [[Complex32;2];3],
    buffer: Vec<Complex32>,
    chirp_buffer: Vec<Complex32>,
    up_chirp: Vec<Complex32>,
    down_chirp: Vec<Complex32>,
    total_items: usize,
    chirp_len: usize,
    fft: Arc<dyn Fft<f32>>,
    state: LoraSyncStateMachine,
}

impl LoraDemodulator {
    pub fn new(center: f64, bandwidth: f64, hz: f64, sf: u32) -> Self {
        let sf2 = 2_u32.pow(sf) as usize;
        let shifter = Shifter::new(-center + bandwidth / 2.0, hz);

        let up_chirp = upchirp(hz, bandwidth, sf);
        let down_chirp = downchirp(hz, bandwidth, sf);
        let chirp_len = up_chirp.len();
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(chirp_len);

        Self {
            shifter,
            center,
            hz,
            bandwidth,
            sf,
            sf2,
            zi: [[Complex32::new(0.0,0.0);2];3],
            up_chirp,
            down_chirp,
            chirp_buffer: vec![],
            buffer: vec![],
            total_items: 0,
            chirp_len,
            fft,
            state: LoraSyncStateMachine::new(sf),
        }
    }

    fn process_buffer(&mut self, mut for_each: impl FnMut(Packet)) {
        assert_eq!(self.buffer.len(), BUFFER_SIZE);

        self.shifter.shift(&mut self.buffer, self.total_items);
        sosfilt(&LOW_PASS_SOS_200K, &mut self.buffer, &mut self.zi);

        let mut unconsumed_view: &[Complex32] = &self.buffer.clone();

        while !unconsumed_view.is_empty() {
            let take = usize::min(self.chirp_len - self.chirp_buffer.len(), unconsumed_view.len());

            self.chirp_buffer.extend_from_slice(&unconsumed_view[..take]);
            unconsumed_view = &unconsumed_view[take..];
            if self.chirp_buffer.len() == self.chirp_len {
                self.process_chirp(&mut for_each);
            }
        }

        self.total_items += self.buffer.len();
        self.buffer.clear();
    }


    fn get_symbol(&self, s: &[Complex<f32>], dechirp: &[Complex<f32>], sf: u32) -> Symbol {
        let mut d = s.to_vec();
        product(&mut d, dechirp);

        self.fft.process(&mut d);
        let sf2 = 2_u32.pow(sf) as usize;

        let mut freqs = Vec::new();
        freqs.resize(sf2, 0.0);
        let mut max_idx = 0;
        for i in 0..sf2 {
            freqs[i] = d[i].norm() + d[d.len() - sf2 + i].norm();
            if freqs[i] > freqs[max_idx] {
                max_idx = i;
            }
        }

        let total_energy: f32 = freqs.iter().sum();
        let signal_energy = freqs[max_idx];
        let noise_energy = total_energy - signal_energy;
        let snr = signal_energy / noise_energy;

        let left_idx = (max_idx + sf2 - 1) % sf2;
        let right_idx = (max_idx + sf2 + 1) % sf2;

        let ldiff = freqs[max_idx] - freqs[left_idx];
        let rdiff = freqs[max_idx] - freqs[right_idx];

        let adjust = ldiff / (ldiff + rdiff) - 0.5;

        return Symbol::new(max_idx, snr, adjust);
    }

    fn process_chirp(&mut self, mut for_each: impl FnMut(Packet)) {
        assert_eq!(self.chirp_buffer.len(), self.chirp_len);

        let dechirp = match self.state.which_chirp() {
            ChirpKind::Up => &self.down_chirp,
            ChirpKind::Down => &self.up_chirp,
        };

        let sym = self.get_symbol(&self.chirp_buffer, dechirp, self.sf);

        let shift_amount = self.state.next(sym, &mut for_each);
        //keep the last shift_amount of the chirp for the next chirp to process
        let shift_idx = ((1.0 - shift_amount) * self.chirp_len as f64) as usize;
        self.chirp_buffer.copy_within(shift_idx..,0);
        self.chirp_buffer.resize(self.chirp_len - shift_idx, Complex32::new(0.0,0.0));
    }

    pub fn feed(&mut self, mut items: &[Complex<f32>], mut for_each: impl FnMut(Packet)) {
        loop {
            let needed = BUFFER_SIZE - self.buffer.len();
            if items.len() > needed {
                self.buffer.extend_from_slice(&items[..needed]);
                items = &items[needed..];
                if self.buffer.len() == BUFFER_SIZE {
                    self.process_buffer(&mut for_each);
                }
            } else {
                self.buffer.extend_from_slice(items);
                if self.buffer.len() == BUFFER_SIZE {
                    self.process_buffer(&mut for_each);
                }
                return;
            }
        }
    }
}
use num_complex::Complex;


pub struct Shifter {
    buffer: Box<[Complex<f32>]>
}


impl Shifter {
    pub fn new(freq: f64, hz: f64) -> Shifter {
        let mut best_repeat_ct = 1;
        let mut best_freq_diff = f32::INFINITY;

        for sin_repeat_ct in 1..10 {
            let signed_sin_len = (hz / freq * sin_repeat_ct as f64).round() / sin_repeat_ct as f64;
            if signed_sin_len == 0.0 {
                break;
            }
            let diff = (hz / signed_sin_len - freq) as f32;
            if diff.abs() < best_freq_diff.abs() {
                best_freq_diff = diff;
                best_repeat_ct = sin_repeat_ct;
            }
        }

        if best_freq_diff.abs() > 0.001 {
            eprintln!("Shift frequency change ({} -> {})", freq, freq as f32 + best_freq_diff);
        }

        let mut signed_sin_len = ((hz / freq * best_repeat_ct as f64) / best_repeat_ct as f64) as f32;
        if signed_sin_len == 0.0 {
            signed_sin_len = 1.0;
        }

        let sin_len = signed_sin_len.abs() as usize;
        let buffer = (0..sin_len).map(|i| {
            let angle = 2.0 * std::f32::consts::PI / (signed_sin_len * best_repeat_ct as f32) * i as f32;
            Complex::from_polar(1.0, angle)
        }).collect();

        Shifter { buffer }
    }

    pub fn shift(&self, x: &mut [Complex<f32>], start: usize) {
        let offset = start % self.buffer.len();

        let mut i = 0;
        for j in offset..self.buffer.len() {
            x[i] *= self.buffer[j];
            i += 1;
            if i == x.len() {
                return;
            }
        }

        let fulls = (x.len() - self.buffer.len() + offset) / self.buffer.len();
        for _ in 0..fulls {
            for j in self.buffer.iter() {
                x[i] *= j;
                i += 1;
            }
        }

        let rest = x.len() - (self.buffer.len() - offset + fulls * self.buffer.len());
        for j in 0..rest {
            x[i] = self.buffer[j];
            i += 1;
        }
    }
}
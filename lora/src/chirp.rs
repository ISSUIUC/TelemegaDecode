use num_complex::Complex;

pub fn upchirp(hz: f64, bandwidth: f64, spreading_factor: u32) -> Vec<Complex<f32>> {
    let sf2 = 2_u32.pow(spreading_factor) as f64;
    let chirp_rate = bandwidth / sf2;
    let chirp_width = (hz / chirp_rate) as usize;

    (0..chirp_width).map(|x|{
        let k = x as f64 * sf2 / chirp_width as f64;
        Complex::from_polar(1.0, (2.0 * std::f64::consts::PI * 0.5 * (k * k / sf2 - 2.0 * k)) as f32)
    }).collect()
}

pub fn downchirp(hz: f64, bandwidth: f64, spreading_factor: u32) -> Vec<Complex<f32>> {
    let sf2 = 2_u32.pow(spreading_factor) as f64;
    let chirp_rate = bandwidth / sf2;
    let chirp_width = (hz / chirp_rate) as usize;

    (0..chirp_width).map(|x|{
        let k = x as f64 * sf2 / chirp_width as f64;
        Complex::from_polar(1.0, (-2.0 * std::f64::consts::PI * 0.5 * (k * k / sf2)) as f32)
    }).collect()
}
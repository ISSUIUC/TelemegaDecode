use num_complex::Complex;

pub fn sosfilt<const N: usize>(sos: &[[f32; 6]; N], x: &mut [Complex<f32>], zi: &mut [[Complex<f32>; 2]; N]) {
    for i in 0..x.len() {
        let mut x_c = x[i];
        for j in 0..N {
            let section = &sos[j];
            let x_n = section[0] * x_c + zi[j][0];
            zi[j][0] = section[1] * x_c - section[4] * x_n + zi[j][1];
            zi[j][1] = section[2] * x_c - section[5] * x_n;
            x_c = x_n;
        }
        x[i] = x_c;
    }
}

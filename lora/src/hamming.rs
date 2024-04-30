const HAMMING1: u8 = 0xd;
const HAMMING2: u8 = 0xb;
const HAMMING4: u8 = 0x7;
const HAMMING6: u8 = 0xe;
const HAMMING8: u8 = 0xff;

fn hamming48_encode(nibble: u8) -> u8 {
    let mut x = nibble;
    if (x & HAMMING1).count_ones() & 1 == 1 {
        x |= 0x80;
    }
    if (x & HAMMING2).count_ones() & 1 == 1 {
        x |= 0x40;
    }
    if (x & HAMMING4).count_ones() & 1 == 1 {
        x |= 0x20;
    }
    if (x & HAMMING8).count_ones() & 1 == 1 {
        x |= 0x10;
    }

    return x;
}

fn hamming45_encode(nibble: u8) -> u8 {
    let mut x = nibble;
    if (x & 0xf).count_ones() & 1 == 1 {
        x |= 0x10;
    }

    return x;
}

fn hamming46_encode(nibble: u8) -> u8 {
    let mut x = nibble;
    if (x & 0xe).count_ones() & 1 == 1 {
        x |= 0x20;
    }
    if (x & 0x7).count_ones() & 1 == 1 {
        x |= 0x10;
    }

    return x;
}


fn hamming47_encode(nibble: u8) -> u8 {
    let mut x = nibble;
    if (x & 0xb).count_ones() & 1 == 1 {
        x |= 0x40;
    }
    if (x & 0xe).count_ones() & 1 == 1 {
        x |= 0x20;
    }
    if (x & 0x7).count_ones() & 1 == 1 {
        x |= 0x10;
    }

    return x;
}


pub fn hamming4_n_decode<const N: usize>(symbol: u8) -> u8 {
    let best = (0..16).fold(0,|a,b|{
        if hamming_distance(hamming4_n_encode::<N>(a), symbol)
            < hamming_distance(hamming4_n_encode::<N>(b), symbol) {
            a
        } else {
            b
        }
    });

    best
}

pub fn hamming4_n_encode<const N: usize>(nibble: u8) -> u8 {
    match N {
        4 => nibble,
        5 => hamming45_encode(nibble),
        6 => hamming46_encode(nibble),
        7 => hamming47_encode(nibble),
        8 => hamming48_encode(nibble),
        _ => todo!()
    }
}

fn hamming_distance(a: u8, b: u8) -> u32 {
    (a ^ b).count_ones()
}
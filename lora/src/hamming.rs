const HAMMING1: u8 = 0xd;
const HAMMING2: u8 = 0xb;
const HAMMING4: u8 = 0x7;
const HAMMING8: u8 = 0xff;

pub fn hamming48_encode(nibble: u8) -> u8 {
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

pub fn hamming48_decode(symbol: u8) -> u8 {
    let best = (0..16).fold(0,|a,b|{
        if hamming_distance(hamming48_encode(a), symbol)
        < hamming_distance(hamming48_encode(b), symbol) {
            a
        } else {
            b
        }
    });

    return best
}

pub fn hamming_distance(a: u8, b: u8) -> u32 {
    (a ^ b).count_ones()
}
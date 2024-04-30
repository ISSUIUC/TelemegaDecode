// deduced interleaving tables
const INTERLEAVE_TABLE_8: [(usize, usize);64] = [
    (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
    (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (0, 1),
    (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (0, 2), (1, 2),
    (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (0, 3), (1, 3), (2, 3),
    (4, 5), (5, 5), (6, 5), (7, 5), (0, 5), (1, 5), (2, 5), (3, 5),
    (5, 4), (6, 4), (7, 4), (0, 4), (1, 4), (2, 4), (3, 4), (4, 4),
    (6, 6), (7, 6), (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6),
    (7, 7), (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7)
];

const INTERLEAVE_TABLE_5: [(usize, usize);40] = [
    (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
    (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (0, 1),
    (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (0, 2), (1, 2),
    (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (0, 3), (1, 3), (2, 3),
    (4, 4), (5, 4), (6, 4), (7, 4), (0, 4), (1, 4), (2, 4), (3, 4)
];

const INTERLEAVE_TABLE_6: [(usize,usize);48] = [
    (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
    (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (0, 1),
    (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (0, 2), (1, 2),
    (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (0, 3), (1, 3), (2, 3),
    (4, 4), (5, 4), (6, 4), (7, 4), (0, 4), (1, 4), (2, 4), (3, 4),
    (5, 5), (6, 5), (7, 5), (0, 5), (1, 5), (2, 5), (3, 5), (4, 5)
];

const INTERLEAVE_TABLE_7: [(usize,usize);56] = [
    (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0),
    (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (0, 1),
    (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (0, 2), (1, 2),
    (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (0, 3), (1, 3), (2, 3),
    (4, 4), (5, 4), (6, 4), (7, 4), (0, 4), (1, 4), (2, 4), (3, 4),
    (5, 5), (6, 5), (7, 5), (0, 5), (1, 5), (2, 5), (3, 5), (4, 5),
    (6, 6), (7, 6), (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6)
];


fn is_bit_set(x: u8, bit: usize) -> bool {
    (x & (1 << bit)) != 0
}

fn table<const CR: usize>() -> &'static [(usize,usize)]{
    match CR {
        5 => &INTERLEAVE_TABLE_5,
        6 => &INTERLEAVE_TABLE_6,
        7 => &INTERLEAVE_TABLE_7,
        8 => &INTERLEAVE_TABLE_8,
        _ => todo!()
    }
}

fn interleave_pos<const CR: usize>(byte: usize, bit: usize) -> (usize, usize) {
    let t = table::<CR>();
    assert!(byte < CR);
    assert!(bit < 8);

    return t[byte * 8 + bit];
}

//in has CR bits per u8
pub fn interleave_block<const CR: usize>(bytes: [u8;8]) -> [u8;CR] {
    let mut ret = [0u8;CR];

    for outbyte in 0..CR {
        for outbit in 0..8 {
            let (inbyte,inbit) = interleave_pos::<CR>(outbyte,outbit);
            if is_bit_set(bytes[inbyte], inbit) {
                ret[outbyte] |= 1 << outbit;
            }
        }
    }

    return ret;
}

//out has CR bits per u8
pub fn deinterleave_block<const CR: usize>(bytes: [u8;CR]) -> [u8;8] {
    let mut ret = [0u8;8];

    for outbyte in 0..CR {
        for outbit in 0..8 {
            let (inbyte,inbit) = interleave_pos::<CR>(outbyte,outbit);
            if is_bit_set(bytes[outbyte], outbit) {
                ret[inbyte] |= 1 << inbit;
            }
        }
    }
    return ret;
}

pub fn deinterleave<const CR: usize>(bytes: &[u8]) -> Vec<u8> {
    bytes.chunks_exact(CR).flat_map(|c|{
        deinterleave_block::<CR>(c.try_into().unwrap())
    }).collect()
}

pub fn interleave<const CR: usize>(bytes: &[u8]) -> Vec<u8> {
    bytes.chunks_exact(CR).flat_map(|c|{
        interleave_block::<CR>(c.try_into().unwrap())
    }).collect()
}
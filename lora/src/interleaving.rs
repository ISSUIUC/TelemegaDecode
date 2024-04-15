// deduced interleaving table
const INTERLEAVE_TABLE: [(usize, usize);64] = [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (1, 1), (2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (0, 1), (2, 2), (3, 2), (4, 2), (5, 2), (6, 2), (7, 2), (0, 2), (1, 2), (3, 3), (4, 3), (5, 3), (6, 3), (7, 3), (0, 3), (1, 3), (2, 3), (4, 5), (5, 5), (6, 5), (7, 5), (0, 5), (1, 5), (2, 5), (3, 5), (5, 4), (6, 4), (7, 4), (0, 4), (1, 4), (2, 4), (3, 4), (4, 4), (6, 6), (7, 6), (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (7, 7), (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7)];


fn is_bit_set(x: u8, bit: usize) -> bool {
    (x & (1 << bit)) != 0
}

pub fn interleave_block(bytes: [u8;8]) -> [u8;8] {
    let mut ret = [0u8;8];

    for outbyte in 0..8 {
        for outbit in 0..8 {
            let (inbyte,inbit) = INTERLEAVE_TABLE[outbyte*8+outbit];
            if is_bit_set(bytes[inbyte], inbit) {
                ret[outbyte] |= (1 << outbit);
            }
        }
    }

    return ret;
}

pub fn deinterleave_block(bytes: [u8;8]) -> [u8;8] {
    let mut ret = [0u8;8];

    for outbyte in 0..8 {
        for outbit in 0..8 {
            let (inbyte,inbit) = INTERLEAVE_TABLE[outbyte*8+outbit];
            if is_bit_set(bytes[outbyte], outbit) {
                ret[inbyte] |= (1 << inbit);
            }
        }
    }
    return ret;
}

pub fn deinterleave(bytes: &[u8]) -> Vec<u8> {
    bytes.chunks_exact(8).flat_map(|c|{
        deinterleave_block(c.try_into().unwrap())
    }).collect()
}

pub fn interleave(bytes: &[u8]) -> Vec<u8> {
    bytes.chunks_exact(8).flat_map(|c|{
        interleave_block(c.try_into().unwrap())
    }).collect()
}
pub fn to_gray(symbol: u8) -> u8 {
    (symbol >> 1) ^ symbol
}

pub fn from_gray(symbol: u8) -> u8 {
    let x = symbol ^ (symbol >> 4);
    let x = x ^ (x >> 2);
    let x = x ^ (x >> 1);

    return x;
}
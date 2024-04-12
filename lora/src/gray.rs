pub fn to_gray(symbol: u16) -> u16 {
    (symbol >> 1) ^ symbol
}

pub fn from_gray(symbol: u16) -> u16 {
    let x = symbol ^ (symbol >> 16);
    let x = x ^ (x >> 8);
    let x = x ^ (x >> 4);
    let x = x ^ (x >> 2);
    let x = x ^ (x >> 1);

    return x;
}
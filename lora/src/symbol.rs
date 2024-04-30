use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub struct Symbol {
    pub value: usize,
    pub snr: f32,
    pub adj: f32,
}

impl Symbol {
    pub fn new(value: usize, snr: f32, adj: f32) -> Self {
        Self {
            value,
            snr,
            adj,
        }
    }
}

pub struct Packet {
    pub symbols: Vec<Symbol>
}

impl Packet {
    pub fn new(symbols: Vec<Symbol>) -> Self {
        Self {
            symbols
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}", self.symbols.iter().map(|s|s.value).collect::<Vec<_>>())
    }
}
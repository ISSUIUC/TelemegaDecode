use itertools::Itertools;

pub fn nibble(x: &[u8]) -> Vec<u8> {
    x.iter().flat_map(|x|[x%16,x/16]).collect()
}

pub fn denibble(x: &[u8]) -> Vec<u8> {
    x.iter().tuples().map(|(low,high)|{
        assert!(*low < 16);
        assert!(*high < 16);
        low + high * 16
    }).collect()
}
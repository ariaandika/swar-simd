const CHUNK_SIZE: usize = size_of::<usize>();

const LSB: usize = usize::from_ne_bytes([1; CHUNK_SIZE]);
const MSB: usize = usize::from_ne_bytes([128; CHUNK_SIZE]);

#[allow(unused)]
macro_rules! logb {
    ($($tt:tt)*) => {{
        let res = $($tt)*;
        for b in res.to_ne_bytes() {
            print!("{b:0>8b} ");
        }
        println!();
        res
    }};
}

pub fn find(mut value: &[u8], byte: u8) -> Option<usize> {
    let start = value.as_ptr();
    let map = |pos| distance(start, value.as_ptr()) + pos;
    loop {
        if let Some((&chunk, rest)) = value.split_first_chunk() {
            match find_block(chunk, byte).map(map) {
                Some(ok) => return Some(ok),
                None => value = rest,
            }
        } else {
            return value.iter().position(|e| e == &byte).map(map);
        }
    }
}

fn find_block(chunk: [u8; CHUNK_SIZE], byte: u8) -> Option<usize> {
    let x = usize::from_ne_bytes(chunk);
    let target = usize::from_ne_bytes([byte; CHUNK_SIZE]);

    let xor_x = x ^ target;
    let found = xor_x.wrapping_sub(LSB) & !xor_x & MSB;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}

pub fn find_nul(chunk: &[u8; CHUNK_SIZE]) -> Option<usize> {
    let x = usize::from_ne_bytes(*chunk);

    let x7 = x.wrapping_sub(LSB);
    let found = x7 & !x & MSB;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}

/// Find the first byte that less than
pub fn find_lt(chunk: [u8; CHUNK_SIZE], byte: u8) -> Option<usize> {
    let x = usize::from_ne_bytes(chunk);
    let b = usize::from_ne_bytes([byte; CHUNK_SIZE]);

    let eq_b = x.wrapping_sub(b) & !x;
    let found = eq_b & MSB;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}

fn distance(start: *const u8, end: *const u8) -> usize {
    unsafe { usize::try_from(end.offset_from(start)).unwrap_unchecked() }
}

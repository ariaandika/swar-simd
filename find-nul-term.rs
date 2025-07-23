const CHUNK_SIZE: usize = size_of::<usize>();

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

fn main() {
    let value = b"Simd Sw0";
    assert!(find_nul(value).is_none());

    let value = b"Simd\0 Sw";
    let i = find_nul(value).unwrap();
    assert_eq!(value[i], b'\0');

    let value = b"Simd Sw\0";
    let i = find_nul(value).unwrap();
    assert_eq!(value[i], b'\0');

    let value = b"\0Simd Sw";
    let i = find_nul(value).unwrap();
    assert_eq!(value[i], b'\0');

    //

    let value = b"Simd Sw0";
    assert!(find_nul_v2(value).is_none());

    let value = b"Simd\0 Sw";
    let i = find_nul_v2(value).unwrap();
    assert_eq!(value[i], b'\0');

    let value = b"Simd Sw\0";
    let i = find_nul_v2(value).unwrap();
    assert_eq!(value[i], b'\0');

    let value = b"\0Simd Sw";
    let i = find_nul_v2(value).unwrap();
    assert_eq!(value[i], b'\0');

    //

    let value = [124, 125, 126, 220, 127, 128, 129, 121];
    assert!(find_lt(value, 100).is_none());

    let value = [124, 125, 126, 20, 127, 128, 129, 11];
    let i = find_lt(value, 100).unwrap();
    assert!(value[i] < 100);

    let value = [124, 125, 126, 133, 127, 128, 129, 20];
    let i = find_lt(value, 100).unwrap();
    assert!(value[i] < 100);

    let value = [24, 125, 126, 133, 127, 128, 129, 130];
    let i = find_lt(value, 100).unwrap();
    assert!(value[i] < 100);
}

fn find_nul(chunk: &[u8; CHUNK_SIZE]) -> Option<usize> {
    let x = usize::from_ne_bytes(*chunk);

    let x2 = x | x << 1;
    let x4 = x2 | x2 << 2;
    let x8 = x4 | x4 << 4;
    let found = !x8 & 0x8080808080808080;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}

fn find_nul_v2(chunk: &[u8; CHUNK_SIZE]) -> Option<usize> {
    let x = usize::from_ne_bytes(*chunk);

    let x7 = x.wrapping_sub(0x0101010101010101);
    let found = x7 & !x & 0x8080808080808080;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}

/// Find the first byte that less than
fn find_lt(chunk: [u8; CHUNK_SIZE], byte: u8) -> Option<usize> {
    const MSB: usize = usize::from_ne_bytes([128; CHUNK_SIZE]);

    let bm = usize::from_ne_bytes([byte; CHUNK_SIZE]);
    let chunk = usize::from_ne_bytes(chunk);
    let eq_b = chunk.wrapping_sub(bm) & !chunk;
    let found = eq_b & MSB;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}

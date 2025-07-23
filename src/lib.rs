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

pub fn find(value: &[u8], byte: u8) -> Option<usize> {
    unsafe { find_raw(value.as_ptr(), value.as_ptr().add(value.len()), byte) }
}

/// Find byte between raw pointer.
///
/// # Safety
///
/// The `start` pointer must be valid until the pointer right before `end`. The end pointer is
/// exclusive, just one byte after the last byte.
pub unsafe fn find_raw(start: *const u8, end: *const u8, byte: u8) -> Option<usize> {
    const CHUNK_SIZE: usize = size_of::<usize>();
    const LSB: usize = usize::from_ne_bytes([1; CHUNK_SIZE]);
    const MSB: usize = usize::from_ne_bytes([128; CHUNK_SIZE]);

    let target = usize::from_ne_bytes([byte; CHUNK_SIZE]);
    let mut current = start;

    loop {
        unsafe {
            let next = current.add(CHUNK_SIZE);
            if next > end {
                break;
            }

            // SWAR
            let x = usize::from_ne_bytes(*current.cast());

            let xor_x = x ^ target;
            let found = xor_x.wrapping_sub(LSB) & !xor_x & MSB;

            if found != 0 {
                return Some(
                    usize::try_from(current.offset_from(start)).unwrap_unchecked()
                        + (found.trailing_zeros() / 8) as usize,
                );
            }

            current = next;
        }
    }

    while current < end {
        unsafe {
            if *current == byte {
                return Some(usize::try_from(current.offset_from(start)).unwrap_unchecked());
            }
            current = current.add(1);
        }
    }

    None
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

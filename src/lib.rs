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

/// Find byte in a bytes.
#[inline]
pub fn find(value: &[u8], byte: u8) -> Option<usize> {
    unsafe { find_raw(value.as_ptr(), value.as_ptr().add(value.len()), byte) }
}

/// Find byte between raw pointer.
///
/// # Safety
///
/// The `start` pointer must be valid until the pointer right before `end`.
pub unsafe fn find_raw(start: *const u8, end: *const u8, byte: u8) -> Option<usize> {
    let target = usize::from_ne_bytes([byte; CHUNK_SIZE]);
    let max = end as usize;
    let mut current = start;

    loop {
        let next = unsafe { (current as usize).unchecked_add(CHUNK_SIZE) };
        if next > max {
            break;
        }

        // SWAR
        let x = usize::from_ne_bytes(unsafe { *current.cast() });

        let xor_x = x ^ target;
        let found = xor_x.wrapping_sub(LSB) & !xor_x & MSB;

        if found != 0 {
            let pos = (found.trailing_zeros() / 8) as usize;
            let offset = unsafe { current.offset_from(start) as usize };

            return Some(unsafe { offset.unchecked_add(pos) });
        }

        current = next as _;
    }

    while current < end {
        if unsafe { *current } == byte {
            return Some(unsafe { current.offset_from(start) as usize });
        }
        current = unsafe { current.add(1) };
    }

    None
}

/// Find byte in a bytes.
#[inline]
pub fn find2(value: &[u8], b1: u8, b2: u8) -> Option<usize> {
    unsafe { find2_raw(value.as_ptr(), value.as_ptr().add(value.len()), b1, b2) }
}

/// Find 2 byte between raw pointer.
///
/// # Safety
///
/// The `start` pointer must be valid until the pointer right before `end`.
pub unsafe fn find2_raw(start: *const u8, end: *const u8, b1: u8, b2: u8) -> Option<usize> {
    let t1 = usize::from_ne_bytes([b1; CHUNK_SIZE]);
    let t2 = usize::from_ne_bytes([b2; CHUNK_SIZE]);
    let max = end as usize;
    let mut current = start;

    loop {
        let next = unsafe { (current as usize).unchecked_add(CHUNK_SIZE) };
        if next > max {
            break;
        }

        // SWAR
        let x = usize::from_ne_bytes(unsafe { *current.cast() });

        let xor_x = x ^ t1;
        let found_x = xor_x.wrapping_sub(LSB) & !xor_x;

        let xor_y = x ^ t2;
        let found_y = xor_y.wrapping_sub(LSB) & !xor_y;

        let found = (found_x | found_y) & MSB;

        if found != 0 {
            let pos = (found.trailing_zeros() / 8) as usize;
            let offset = unsafe { current.offset_from(start) as usize };

            return Some(unsafe { offset.unchecked_add(pos) });
        }

        current = next as _;
    }

    while current < end {
        if unsafe { *current } == b1 {
            return Some(unsafe { current.offset_from(start) as usize });
        }
        current = unsafe { current.add(1) };
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

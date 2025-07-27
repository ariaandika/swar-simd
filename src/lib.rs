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

        let x = usize::from_ne_bytes(unsafe { *current.cast() });

        // SWAR
        //
        // `x ^ target` all matching bytes will be 0x00
        //
        // `xor_x.wrapping_sub(LSB)` matching bytes will wrap to 0xFF
        // `!xor_x` matching bytes will be 0xFF
        //
        // bitwise AND both, resulting:
        // - matched byte to be 0xFF
        // - non-matched to be 0x00
        //
        // bitwise AND with MSB, resulting only the most
        // significant bit of the matched byte to be set
        //
        // if no match found, all bytes will be 0x00
        //
        // otherwise, `.trailing_zeros() / 8` returns
        // the first byte index that is matched

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

/// Find the first either 2 byte in a bytes.
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

/// Find the first byte that less than target byte.
///
/// This function cannot find target byte that more than 128.
///
/// # Panics
///
/// Panics if `target > 128`.
pub fn find_lt(chunk: [u8; CHUNK_SIZE], target: u8) -> Option<usize> {

    let x = usize::from_ne_bytes(chunk);
    let b = usize::from_ne_bytes([target; CHUNK_SIZE]);

    // SWAR
    //
    // # `x.wrapping_sub(b)`
    //
    // the goal is to toggle *target bytes MSB* by subtraction.
    //
    // bytes less than target, will wraped, thus toggling MSB.
    //
    // # `!x`
    //
    // this will toggle all bytes MSB
    //
    // # `x.wrapping_sub(b) & !x`
    //
    // the *target bytes* will keep the MSB, while other will unset the MSB
    //
    // # `& MSB`
    //
    // this will only set the most significant byte.
    //
    // if no byte found, the result will be exactly 0
    //
    // otherwise, any byte found will have its most significant byte set
    //
    // # `trailing_zeros() / 8`
    //
    // `trailing_zeros()` will returns the amount unset bit until the first set bit,
    // which the byte that equal to target.
    //
    // dividing it by 8, returns the index of found byte

    // # Edge Case
    //
    // If `target` is `> 128` and the found byte is also `> 128`,
    // this algorithm will choke.
    //
    // the found byte will be toggled twice, when subtracting pass `128`
    // and wrapping to 255, which is wrong.
    assert!(target <= 128);

    let found = x.wrapping_sub(b) & !x & MSB;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}


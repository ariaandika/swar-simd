#[allow(unused_imports)]
use super::{logb, logbln, logbyln};

const BLOCK: usize = size_of::<usize>();
const LSB: usize = usize::from_ne_bytes([1; BLOCK]);
const MSB: usize = usize::from_ne_bytes([128; BLOCK]);

const CR: usize = usize::from_ne_bytes([b'\r'; BLOCK]);
const LF: usize = usize::from_ne_bytes([b'\n'; BLOCK]);
const SP: usize = usize::from_ne_bytes([b' '; BLOCK]);
const DEL: usize = usize::from_ne_bytes([127; 8]);

/// Find byte in a bytes.
#[inline]
pub fn find(value: &[u8], byte: u8) -> Option<usize> {
    let target = usize::from_ne_bytes([byte; BLOCK]);
    let start = value.as_ptr();
    let end = unsafe { value.as_ptr().add(value.len()) };
    let max = end.addr();

    let mut current = value.as_ptr();

    while current.addr() + BLOCK <= max {
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

        // # Optimize
        //
        // if the block is checked that it is `< 128`,
        // `& !xor_x` can be dropped

        let block = usize::from_ne_bytes(unsafe { *current.cast() });

        let xor_x = block ^ target;
        let result = xor_x.wrapping_sub(LSB) & !xor_x & MSB;

        if result != 0 {
            let pos = (result.trailing_zeros() / 8) as usize;
            let offset = unsafe { current.offset_from_unsigned(start) };

            return Some(unsafe { offset.unchecked_add(pos) });
        }

        current = unsafe { current.add(BLOCK) }
    }

    while current < end {
        if unsafe { *current } == byte {
            return Some(unsafe { current.offset_from_unsigned(start) });
        }
        current = unsafe { current.add(1) };
    }

    None
}

/// Find the first either 2 byte in a bytes.
#[inline]
pub fn find2(value: &[u8], b1: u8, b2: u8) -> Option<usize> {
    let t1 = usize::from_ne_bytes([b1; BLOCK]);
    let t2 = usize::from_ne_bytes([b2; BLOCK]);
    let start = value.as_ptr();
    let end = unsafe { value.as_ptr().add(value.len()) };
    let max = end.addr();

    let mut current = start;

    while current.addr() + BLOCK <= max {
        let block = usize::from_ne_bytes(unsafe { *current.cast() });

        let xor_1 = block ^ t1;
        let result_1 = xor_1.wrapping_sub(LSB) & !xor_1;

        let xor_2 = block ^ t2;
        let result_2 = xor_2.wrapping_sub(LSB) & !xor_2;

        let result = (result_1 | result_2) & MSB;
        if result != 0 {
            let pos = (result.trailing_zeros() / 8) as usize;
            let offset = unsafe { current.offset_from_unsigned(start) };

            return Some(unsafe { offset.unchecked_add(pos) });
        }

        current = unsafe { current.add(BLOCK) }
    }

    while current < end {
        if unsafe { *current } == b1 {
            return Some(unsafe { current.offset_from_unsigned(start) });
        }
        current = unsafe { current.add(1) };
    }

    None
}

// b' '..=b'~'
//
// can be used as search in between byte range
pub fn find_non_printable_ascii(value: &[u8]) -> Option<usize> {
    let start = value.as_ptr();
    let end = unsafe { start.add(value.len()) };
    let max = end.addr();

    let mut current = start;

    while current.addr() + BLOCK <= max {
        // SWAR
        let block = usize::from_ne_bytes(unsafe { *current.cast() });

        // byte != 127(DEL)
        let not_del = block ^ DEL;
        let not_del = not_del.wrapping_sub(LSB) & !not_del;

        // 32(SP) < byte
        let lt_32 = block.wrapping_sub(SP) & !block;

        // NOTE:
        // if MSB is set on `block`, value is >= 128

        let result = (block | not_del | lt_32) & MSB;
        if result != 0 {
            let pos = (result.trailing_zeros() / 8) as usize;
            let offset = unsafe { current.offset_from_unsigned(start) };
            return Some(offset + pos);
        }

        current = unsafe { current.add(BLOCK) }
    }

    while current < end {
        unsafe {
            if !matches!(*current, b' '..=b'~') {
                return Some(current.offset_from_unsigned(start));
            }
            current = current.add(1);
        }
    }

    None
}


// Specific


// combine multiple matcher
#[inline]
pub fn find_combine(value: &[u8]) -> Option<usize> {
    let start = value.as_ptr();
    let end = unsafe { start.add(value.len()) };
    let max = end.addr();

    let mut current = start;

    while current.addr() + BLOCK <= max {
        let block = usize::from_ne_bytes(unsafe { *current.cast() });

        // look for "\r"
        let is_cr = block ^ CR;
        let is_cr = is_cr.wrapping_sub(LSB) & !is_cr;

        // look for "\n"
        let is_lf = block ^ LF;
        let is_lf = is_lf.wrapping_sub(LSB) & !is_lf;

        // look for 127(DEL)
        let not_del = block ^ DEL;
        let not_del = not_del.wrapping_sub(LSB) & !not_del;

        // 32(SP) <= byte, if sub wrapped, MSB is set
        let lt_32 = block.wrapping_sub(SP) & !block;

        // NOTE:
        // if MSB is set on `block`, value is >= 128

        let result = (block | is_cr | is_lf | not_del | lt_32) & MSB;
        if result != 0 {
            let pos = (result.trailing_zeros() / 8) as usize;
            let offset = unsafe { current.offset_from_unsigned(start) };
            return Some(offset + pos);
        }

        current = unsafe { current.add(BLOCK) };
    }

    unsafe {
        find_combine_scalar(std::slice::from_raw_parts(
            current,
            end.offset_from_unsigned(current),
        ))
    }
}

#[inline]
pub fn find_combine_scalar(value: &[u8]) -> Option<usize> {
    value
        .iter()
        .position(|b| matches!(b, b'\n' | b'\r') || !matches!(b, b'!'..=b'~'))
}

/// Find the first byte that less than target byte.
///
/// This function cannot find target byte that more than 128.
///
/// # Panics
///
/// Panics if `target > 128`.
pub fn find_lt(chunk: [u8; BLOCK], target: u8) -> Option<usize> {
    let x = usize::from_ne_bytes(chunk);
    let b = usize::from_ne_bytes([target; BLOCK]);

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
    // if the value have its MSB set (>= 128), it will be unset,
    // thus will never be selected becase `x & 0` is always 0.
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

pub fn find_lt_128(chunk: [u8; BLOCK]) -> Option<usize> {
    let x = usize::from_ne_bytes(chunk);

    // if the MSB is set, then `byte >= 128`
    let found = x & MSB;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}

pub fn find_nul(chunk: &[u8; BLOCK]) -> Option<usize> {
    let x = usize::from_ne_bytes(*chunk);

    let x7 = x.wrapping_sub(LSB);
    let found = x7 & !x & MSB;

    if found == 0 {
        None
    } else {
        Some((found.trailing_zeros() / 8) as usize)
    }
}

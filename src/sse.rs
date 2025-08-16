
/// Find byte in a bytes.
#[inline]
pub fn find(value: &[u8], byte: u8) -> Option<usize> {
    unsafe { find_raw(value.as_ptr(), value.as_ptr().add(value.len()), byte) }
}

#[target_feature(enable = "sse4.2")]
fn find_raw(start: *const u8, end: *const u8, byte: u8) -> Option<usize> {
    const CHUNK_SSE_SIZE: usize = size_of::<usize>();
    const CHUNK_SSE_ISIZE: isize = size_of::<usize>() as isize;

    use std::arch::x86_64::*;

    let target = _mm_set1_epi8(byte as i8);
    let mut current = start;

    unsafe {
        while end.offset_from(current) > CHUNK_SSE_ISIZE {
            let chunk = _mm_loadu_si128(current as _);

            let cmp = _mm_cmpeq_epi8(chunk, target);

            let mask = _mm_movemask_epi8(cmp) as u32;

            if mask != 0 {
                // Find the first set bit (index of the match)
                // let pos = mask.trailing_zeros() as usize;
                // return Some(offset + pos);

                let pos = mask.trailing_zeros() as usize;
                let offset = current.offset_from(start) as usize;

                return Some(offset.unchecked_add(pos));
            }

            current = current.add(CHUNK_SSE_SIZE);
        }
    }

    None
}


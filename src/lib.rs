pub mod swar;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse;

// Reference

const CHUNK_SIZE: usize = size_of::<usize>();

#[allow(unused)]
macro_rules! logb {
    ($($tt:tt)*) => {{
        let res = $($tt)*;
        for b in res.to_ne_bytes() {
            print!("{b:0>8b} ");
        }
        res
    }};
}

#[allow(unused)]
macro_rules! logbln {
    ($($tt:tt)*) => {{
        let res = $($tt)*;
        for b in res.to_ne_bytes() {
            print!("{b:0>8b} ");
        }
        println!();
        res
    }};
}

#[allow(unused, reason = "for reference")]
fn iterate_ptr(start: *const u8, end: *const u8) {
    let mut current = start;

    unsafe {
        while current < end {
            let value = *current;


            current = current.add(1);
        }
    }
}

#[allow(unused, reason = "for reference")]
fn enumerate_ptr(start: *const u8, end: *const u8) {
    unsafe {
        let len = end.offset_from_unsigned(start);
        let mut i = 0;

        while i < len {
            let value = *start.add(i);


            i = i.unchecked_add(1);
        }
    }
}

#[allow(unused, reason = "for reference")]
fn iterate_chunk_ptr(start: *const u8, end: *const u8) {
    let mut current = start;

    unsafe {
        while end.offset_from_unsigned(current) >= CHUNK_SIZE {
            let chunk = *current.cast::<[u8; CHUNK_SIZE]>();


            current = current.add(CHUNK_SIZE);
        }
    }
}


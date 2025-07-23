const CHUNK_SIZE: usize = size_of::<usize>();

fn main() {
    let value = b"Si\0md Sw";

    let i = find_nul(value).unwrap();

    assert_eq!(value[i], b'\0');
}

fn find_nul(mut bytes: &[u8]) -> Option<usize> {
    loop {
        if let Some((chunk, rest)) = bytes.split_first_chunk() {
            if let Some(ok) = find_nul_chunk(chunk) {
                return Some(ok);
            }
            bytes = rest;
        } else {
            return bytes.iter().position(|e| e == &0);
        }
    }
}

fn find_nul_chunk(chunk: &[u8; CHUNK_SIZE]) -> Option<usize> {
    let x = usize::from_ne_bytes(*chunk);

    let x2 = x | x << 1;
    let x4 = x2 | x2 << 2;
    let x8 = x4 | x4 << 4;
    let byte_map = !x8 & 0x8080808080808080;

    if byte_map == 0 {
        None
    } else {
        Some((byte_map.trailing_zeros() / 8) as usize)
    }
}

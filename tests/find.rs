#[test]
fn test_find() {
    use swar_simd::swar::find;

    let value = b"Simd  Swar Example";
    assert!(find(value, b'c').is_none());

    let value = b"Simd  Swar Example";
    let i = find(value, b'd').unwrap();
    assert_eq!(value[i], b'd');

    let value = b"Simd  Swar Example";
    let i = find(value, b'w').unwrap();
    assert_eq!(value[i], b'w');

    let value = b"Simd  Swar Example";
    let i = find(value, b'l').unwrap();
    assert_eq!(value[i], b'l');

    let value = b"Simd  Swar Example";
    let i = find(value, b'e').unwrap();
    assert_eq!(value[i], b'e');

    let value = b"Simd  Swar Example";
    let i = find(value, b'S').unwrap();
    assert_eq!(value[i], b'S');
}

#[test]
fn test_find_non_printable_ascii() {
    use swar_simd::swar::find_non_printable_ascii as find;

    // multiply of pointer size
    assert!(find(&[b' '; 16]).is_none());
    assert!(find(&[b'~'; 16]).is_none());

    assert_eq!(find(&[0; 16]), Some(0));
    assert_eq!(find(&[0b0111_1111; 16]), Some(0));
    assert_eq!(find(&[0b1000_0000; 16]), Some(0));
    assert_eq!(find(&[u8::MAX; 16]), Some(0));

    let mut bytes = [b' '; 16];
    bytes[2] = 31;
    assert_eq!(find(&bytes), Some(2));
    bytes[2] = 0b0111_1111;
    assert_eq!(find(&bytes), Some(2));
    bytes[2] = 0b1000_0000;
    assert_eq!(find(&bytes), Some(2));
    bytes[2] = u8::MAX;
    assert_eq!(find(&bytes), Some(2));

    let mut bytes = [b' '; 16];
    *bytes.last_mut().unwrap() = 31;
    assert_eq!(find(&bytes), Some(15));
    *bytes.last_mut().unwrap() = 0b0111_1111;
    assert_eq!(find(&bytes), Some(15));
    *bytes.last_mut().unwrap() = 0b1000_0000;
    assert_eq!(find(&bytes), Some(15));
    *bytes.last_mut().unwrap() = u8::MAX;
    assert_eq!(find(&bytes), Some(15));

    // non multiply of pointer size
    assert!(find(&[b' '; 15]).is_none());
    assert!(find(&[b'~'; 15]).is_none());

    assert_eq!(find(&[0; 15]), Some(0));
    assert_eq!(find(&[0b0111_1111; 15]), Some(0));
    assert_eq!(find(&[0b1000_0000; 15]), Some(0));
    assert_eq!(find(&[u8::MAX; 15]), Some(0));

    let mut bytes = [b' '; 15];
    bytes[9] = 31;
    assert_eq!(find(&bytes), Some(9));
    bytes[9] = 0b0111_1111;
    assert_eq!(find(&bytes), Some(9));
    bytes[9] = 0b1000_0000;
    assert_eq!(find(&bytes), Some(9));
    bytes[9] = u8::MAX;
    assert_eq!(find(&bytes), Some(9));

    let mut bytes = [b' '; 15];
    bytes[14] = 31;
    assert_eq!(find(&bytes), Some(14));
    bytes[14] = 0b0111_1111;
    assert_eq!(find(&bytes), Some(14));
    bytes[14] = 0b1000_0000;
    assert_eq!(find(&bytes), Some(14));
    bytes[14] = u8::MAX;
    assert_eq!(find(&bytes), Some(14));
}

#[test]
fn test_find_sse() {
    use swar_simd::sse::find;

    let value = b"Simd  Swar Example";
    assert!(find(value, b'c').is_none());

    let value = b"Simd  Swar Example";
    let i = find(value, b'd').unwrap();
    assert_eq!(value[i], b'd');

    let value = b"Simd  Swar Example";
    let i = find(value, b'w').unwrap();
    assert_eq!(value[i], b'w');

    let value = b"Simd  Swar Example";
    let i = find(value, b'l').unwrap();
    assert_eq!(value[i], b'l');

    let value = b"Simd  Swar Example";
    let i = find(value, b'e').unwrap();
    assert_eq!(value[i], b'e');

    let value = b"Simd  Swar Example";
    let i = find(value, b'S').unwrap();
    assert_eq!(value[i], b'S');
}

#[test]
fn test_find2() {
    use swar_simd::swar::find2;

    let value = b"Simd  Swar Example";
    assert!(find2(value, b'c', b'z').is_none());

    let value = b"Simd  Swar Example";
    let i = find2(value, b'd', b'r').unwrap();
    assert_eq!(value[i], b'd');

    let value = b"Simd  Swar Example";
    let i = find2(value, b'r', b'd').unwrap();
    assert_eq!(value[i], b'd');

    let value = b"Simd  Swar Example";
    let i = find2(value, b'l', b'g').unwrap();
    assert_eq!(value[i], b'l');

    let value = b"Simd  Swar Example";
    let i = find2(value, b'e', b'g').unwrap();
    assert_eq!(value[i], b'e');

    let value = b"Simd  Swar Example";
    let i = find2(value, b'S', b'S').unwrap();
    assert_eq!(value[i], b'S');
}

#[test]
fn test_find_nul() {
    use swar_simd::swar::find_nul;

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
}

#[test]
fn test_find_lt() {
    use swar_simd::swar::find_lt;

    macro_rules! max_array_with {
        ($val:expr, $at:expr) => {{
            let mut a = [255u8; 8];
            a[$at] = $val;
            a
        }};
    }

    let value = max_array_with!(155, 5);
    assert!(find_lt(value, 100).is_none());

    let value = max_array_with!(20, 5);
    let i = find_lt(value, 100).unwrap();
    assert!(value[i] == 20);

    let value = max_array_with!(20, 0);
    let i = find_lt(value, 100).unwrap();
    assert!(value[i] == 20);

    let value = max_array_with!(20, 7);
    let i = find_lt(value, 100).unwrap();
    assert!(value[i] == 20);
}


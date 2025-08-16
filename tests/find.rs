#[test]
fn test_find() {
    use swar_simd::find;

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
fn test_find_sse() {
    use swar_simd::find_sse as find;

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
    use swar_simd::find2;

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
    use swar_simd::find_nul;

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
    use swar_simd::find_lt;

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

#[test]
fn test_only() {
    use swar_simd::byte_between_32_to_127;

    for byte in 32..127 {
        assert!(byte_between_32_to_127(byte));
    }
    for byte in 0..32 {
        assert!(!byte_between_32_to_127(byte));
    }
    for byte in 127..=255 {
        assert!(!byte_between_32_to_127(byte));
    }
}


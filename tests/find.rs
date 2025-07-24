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

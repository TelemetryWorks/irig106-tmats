//! L3 span checks, independent of scanner/interpretation rules.

use irig106_tmats::Span;

/// Requirements: L3-ROB-001
#[test]
fn span_constructor_checks_boundaries() {
    let cases = [
        (0, 0, 0, true),
        (0, 1, 0, false),
        (1, 0, 0, false),
        (0, 4, 4, true),
        (4, 0, 4, true),
        (4, 1, 4, false),
        (5, 0, 4, false),
        (1, 3, 4, true),
        (1, 4, 4, false),
        (usize::MAX, 0, usize::MAX, true),
        (usize::MAX, 1, usize::MAX, false),
        (usize::MAX - 1, 1, usize::MAX, true),
        (usize::MAX - 1, 2, usize::MAX, false),
        (0, usize::MAX, usize::MAX, true),
        (1, usize::MAX, usize::MAX, false),
    ];
    for (start, len, source_len, expected) in cases {
        assert_eq!(
            Span::new(start, len, source_len).is_some(),
            expected,
            "start={start}, len={len}, source_len={source_len}"
        );
    }
}

/// Requirements: L3-ROB-002
#[test]
fn span_access_rechecks_actual_source() {
    let source = b"abcd";
    let span = Span::new(2, 2, 32).expect("valid descriptor for longer source");
    assert_eq!(span.get(source), Some(&b"cd"[..]));
    assert_eq!(span.get(b"abc"), None);
    assert_eq!(span.get(b""), None);
    let end = Span::new(4, 0, 4).expect("empty at end");
    assert_eq!(end.get(source), Some(&b""[..]));
    assert_eq!(end.get(b"abc"), None);
    assert_eq!(Span::new(0, 0, 0).expect("empty").get(b""), Some(&b""[..]));
    let huge = Span::new(usize::MAX, 0, usize::MAX).expect("integer-only descriptor");
    assert_eq!(huge.get(source), None);
}

/// Requirements: L3-ROB-001, L3-ROB-002
#[test]
fn every_small_valid_interval_returns_the_original_bytes() {
    let original = [0, 255, 128, b':', b';', b'\r', b'\n', b' '];
    for source_len in 0..=original.len() {
        let source = &original[..source_len];
        for start in 0..=source_len {
            for end in start..=source_len {
                let span = Span::new(start, end - start, source_len).expect("valid interval");
                let borrowed = span.get(source).expect("actual extent fits");
                assert_eq!(borrowed, &source[start..end]);
                assert_eq!(borrowed.as_ptr(), source[start..end].as_ptr());
            }
        }
    }
}

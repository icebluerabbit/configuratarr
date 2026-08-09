//! `SecretValue` — redacted Debug, plaintext expose, drop safety.

use core_lib::SecretValue;

#[test]
fn expose_roundtrips_plaintext() {
    let s = SecretValue::new("hunter2".to_string());
    assert_eq!(s.expose(), "hunter2");
}

#[test]
fn debug_is_redacted() {
    let s = SecretValue::new("hunter2".to_string());
    assert_eq!(format!("{:?}", s), "SecretValue([redacted])");
}

#[test]
fn drop_does_not_panic() {
    let s = SecretValue::new("to_be_dropped".to_string());
    drop(s);
}

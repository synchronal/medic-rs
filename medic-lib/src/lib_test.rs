// @related [subject](medic-lib/src/lib.rs)

use super::*;

#[test]
fn utf8() {
  assert_eq!(std_to_string("héllo".as_bytes().to_vec()), "héllo");
}

#[test]
fn invalid_utf8() {
  assert_eq!(std_to_string(vec![b'a', 0xff, b'b']), "a\u{fffd}b");
}

#[test]
fn empty() {
  assert_eq!(std_to_string(Vec::new()), "");
}

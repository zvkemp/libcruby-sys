use super::*;
use libc::c_int;

extern {
    pub fn rb_enc_get_index(obj: VALUE) -> c_int;
    pub fn rb_utf8_encindex() -> c_int;
}

tests! {
    use super::*;
    use super::super::testing::{Assertions, ToRuby};

    #[test]
    fn test_enc_get_index(assert: &mut Assertions) {
        assert.rs_eq(unsafe { rb_enc_get_index("foo".to_ruby()) } > 0, true);
    }

    #[test]
    fn test_rb_utf8_encindex(assert: &mut Assertions) {
        assert.rs_eq(unsafe { rb_enc_get_index("foo".to_ruby()) }, unsafe { rb_utf8_encindex() });
    }
}

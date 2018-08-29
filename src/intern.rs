use super::*;
use libc::{c_char, c_int, c_long};

extern {
    pub fn rb_ary_new() -> VALUE;
    pub fn rb_ary_new_capa(capacity: c_long) -> VALUE;
    pub fn rb_ary_push(array: VALUE, item: VALUE) -> VALUE;
    pub fn rb_ary_entry(array: VALUE, idx: c_long) -> VALUE;

    pub fn rb_utf8_str_new(ptr: *const c_char, len: c_long) -> VALUE;

    pub fn rb_class_new_instance(argc: c_int, argv: *const VALUE, class: VALUE) -> VALUE;

    pub fn rb_const_get(module: VALUE, name: ID) -> VALUE;

    pub fn rb_inspect(v: VALUE) -> VALUE;
    pub fn rb_obj_class(obj: VALUE) -> VALUE;

    pub fn rb_define_singleton_method(class: VALUE, name: *const c_char, func: ANYARGS<VALUE>, arity: c_int);
}

tests! {
    use super::*;
    use super::super::testing::{Assertions, ToRuby, lazy_eval};
    use std::ptr::null;

    #[test]
    fn test_ary_new(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("[]"), unsafe { rb_ary_new() });

        let arr1 = unsafe { rb_ary_new() };

        unsafe { rb_ary_push(arr1, Qtrue) };
        unsafe { rb_ary_push(arr1, Qfalse) };
        unsafe { rb_ary_push(arr1, Qnil) };

        assert.rb_eq(lazy_eval("[true, false, nil]"), arr1);

        let arr2 = unsafe { rb_ary_new() };

        unsafe { rb_ary_push(arr2, "hello".to_ruby()) };
        unsafe { rb_ary_push(arr2, "world!".to_ruby()) };

        assert.rb_eq(lazy_eval("['hello', 'world!']"), arr2);
    }

    #[test]
    fn test_ary_new_capa(assert: &mut Assertions) {
        assert.rb_eq(lazy_eval("[]"), unsafe { rb_ary_new_capa(0) });
        assert.rb_eq(lazy_eval("[]"), unsafe { rb_ary_new_capa(100) });

        let arr1 = unsafe { rb_ary_new_capa(3) };

        unsafe { rb_ary_push(arr1, Qtrue) };
        unsafe { rb_ary_push(arr1, Qfalse) };
        unsafe { rb_ary_push(arr1, Qnil) };

        assert.rb_eq(lazy_eval("[true, false, nil]"), arr1);

        let arr2 = unsafe { rb_ary_new_capa(0) };

        unsafe { rb_ary_push(arr2, "hello".to_ruby()) };
        unsafe { rb_ary_push(arr2, "world!".to_ruby()) };

        assert.rb_eq(lazy_eval("['hello', 'world!']"), arr2);
    }

    #[test]
    fn test_ary_entry(assert: &mut Assertions) {
        let arr1 = unsafe { rb_ary_new_capa(3) };

        unsafe { rb_ary_push(arr1, Qtrue) };
        unsafe { rb_ary_push(arr1, Qfalse) };
        unsafe { rb_ary_push(arr1, Qnil) };
        unsafe { rb_ary_push(arr1, "hello".to_ruby()) };

        assert.rb_eq(unsafe { Qtrue }, unsafe { rb_ary_entry(arr1, 0) });
        assert.rb_eq(unsafe { Qfalse }, unsafe { rb_ary_entry(arr1, 1) });
        assert.rb_nil(unsafe { rb_ary_entry(arr1, 2) });
        assert.rb_eq("hello".to_ruby(), unsafe { rb_ary_entry(arr1, 3) });
        assert.rb_nil(unsafe { rb_ary_entry(arr1, 4) });
        assert.rb_eq("hello".to_ruby(), unsafe { rb_ary_entry(arr1, -1) });
    }

    #[test]
    fn test_utf8_str_new(assert: &mut Assertions) {
        let static_str = "static str";
        let static_str_ptr = static_str.as_ptr() as *const c_char;

        let heap_string = format!("heap string");
        let heap_string_ptr = heap_string.as_ptr() as *const c_char;

        let unicode = "❤️💛💚💙💜";
        let unicode_ptr = unicode.as_ptr() as *const c_char;

        assert.rb_eq(lazy_eval("''"), unsafe { rb_utf8_str_new(null(), 0) });
        assert.rb_eq(lazy_eval("''"), unsafe { rb_utf8_str_new(static_str_ptr, 0) });
        assert.rb_eq(lazy_eval("''"), unsafe { rb_utf8_str_new(heap_string_ptr, 0) });
        assert.rb_eq(lazy_eval("''"), unsafe { rb_utf8_str_new(unicode_ptr, 0) });

        assert.rb_eq(lazy_eval("'static'"), unsafe { rb_utf8_str_new(static_str_ptr, 6) });
        assert.rb_eq(lazy_eval("'heap'"), unsafe { rb_utf8_str_new(heap_string_ptr, 4) });
        assert.rb_eq(lazy_eval("'❤️'"), unsafe { rb_utf8_str_new(unicode_ptr, 6) });

        assert.rb_eq(lazy_eval("'static str'"), unsafe { rb_utf8_str_new(static_str_ptr, 10) });
        assert.rb_eq(lazy_eval("'heap string'"), unsafe { rb_utf8_str_new(heap_string_ptr, 11) });
        assert.rb_eq(lazy_eval("'❤️💛💚💙💜'"), unsafe { rb_utf8_str_new(unicode_ptr, 22) });
    }

    #[test]
    fn test_class_new_instance(assert: &mut Assertions) {
        assert.rb_ne(lazy_eval("Object.new"), unsafe { rb_class_new_instance(0, null(), rb_cObject) });

        assert.rb_eq(lazy_eval("[]"), unsafe { rb_class_new_instance(0, null(), rb_cArray) });

        let ary_source = unsafe { rb_ary_new_capa(3) };

        unsafe { rb_ary_push(ary_source, Qtrue) };
        unsafe { rb_ary_push(ary_source, Qfalse) };
        unsafe { rb_ary_push(ary_source, Qnil) };

        let ary_cloned = unsafe { rb_class_new_instance(1, &ary_source, rb_cArray) };

        assert.rs_ne(ary_source, ary_cloned);
        assert.rb_eq(lazy_eval("[true, false, nil]"), ary_cloned);

        assert.rb_eq(
            lazy_eval("('a'..'z')"),
            unsafe {
                rb_class_new_instance(
                    2,
                    vec!["a".to_ruby(), "z".to_ruby()].as_ptr(),
                    rb_cRange
                )
            }
        );

        assert.rb_eq(
            lazy_eval("('a'...'z')"),
            unsafe {
                rb_class_new_instance(
                    3,
                    vec!["a".to_ruby(), "z".to_ruby(), Qtrue].as_ptr(),
                    rb_cRange
                )
            }
        );
    }

    #[test]
    fn test_const_get(assert: &mut Assertions) {
        assert.rs_eq(
            unsafe { rb_mKernel },
            unsafe { rb_const_get(rb_cObject, rb_intern(cstr!("Kernel"))) }
        );

        assert.rs_eq(
            unsafe { rb_cObject },
            unsafe { rb_const_get(rb_cObject, rb_intern(cstr!("Object"))) }
        );

        assert.rs_eq(
            unsafe { rb_eEncCompatError },
            unsafe { rb_const_get(rb_cEncoding, rb_intern(cstr!("CompatibilityError"))) }
        );
    }

    #[test]
    fn test_inspect(assert: &mut Assertions) {
        assert.rb_eq(
            lazy_eval("Kernel.inspect"),
            unsafe { rb_inspect(rb_mKernel) }
        );

        assert.rb_eq(
            lazy_eval("Object.inspect"),
            unsafe { rb_inspect(rb_cObject) }
        );

        extern "C" fn __test_inspect__(_self: VALUE) -> VALUE {
            "__test_inspect__ works!".to_ruby()
        }

        let class = unsafe { rb_define_class(cstr!("TestInspect"), rb_cObject) };

        unsafe {
            rb_define_method(
                class,
                cstr!("inspect"),
                ANYARGS::from_arity_1(__test_inspect__),
                0
            );
        }

        assert.rb_eq(
            unsafe { rb_inspect(rb_class_new_instance(0, null(), class)) },
            "__test_inspect__ works!".to_ruby()
        );
    }


    #[test]
    fn test_obj_class(assert: &mut Assertions) {
        let class = unsafe { rb_obj_class(rb_cObject) };
        let module = unsafe { rb_obj_class(rb_mKernel) };
        let instance = unsafe { rb_obj_class(rb_class_new_instance(0, null(), rb_cObject)) };

        assert.rb_eq(lazy_eval("::Class"), class);
        assert.rb_eq(lazy_eval("::Module"), module);
        assert.rb_eq(lazy_eval("::Object"), instance);
    }

    #[test]
    fn test_define_singleton_method(assert: &mut Assertions) {
        extern "C" fn __test_define_singleton_method_arity_0__(_self: VALUE) -> VALUE {
            "__test_define_singleton_method_arity_0__ works!".to_ruby()
        }

        unsafe {
            rb_define_singleton_method(
                rb_cObject,
                cstr!("__test_define_singleton_method_arity_0__"),
                ANYARGS::from_arity_1(__test_define_singleton_method_arity_0__),
                0
            );
        }

        assert.rb_eq(
            lazy_eval("::Object.__test_define_singleton_method_arity_0__"),
            "__test_define_singleton_method_arity_0__ works!".to_ruby()
        );

        extern "C" fn __test_define_singleton_method_arity_3__(_self: VALUE, foo_sym: VALUE, bar_sym: VALUE, baz_sym: VALUE) -> VALUE {
            if unsafe { rb_sym2id(foo_sym) != rb_intern(cstr!("foo")) } {
                "__test_define_singleton_method_arity_3__ failed (expected :foo for first argument)".to_ruby()
            } else if unsafe { rb_sym2id(bar_sym) != rb_intern(cstr!("bar")) } {
                "__test_define_singleton_method_arity_3__ failed (expected :bar for second argument)".to_ruby()
            } else if unsafe { rb_sym2id(baz_sym) != rb_intern(cstr!("baz")) } {
                "__test_define_singleton_method_arity_3__ failed (expected :baz for third argument)".to_ruby()
            } else {
                "__test_define_singleton_method_arity_3__ works!".to_ruby()
            }
        }

        unsafe {
            rb_define_singleton_method(
                rb_cObject,
                cstr!("__test_define_singleton_method_arity_3__"),
                ANYARGS::from_arity_4(__test_define_singleton_method_arity_3__),
                3
            );
        }

        assert.rb_eq(
            lazy_eval("::Object.__test_define_singleton_method_arity_3__(:foo, :bar, :baz)"),
            "__test_define_singleton_method_arity_3__ works!".to_ruby()
        );
    }
}


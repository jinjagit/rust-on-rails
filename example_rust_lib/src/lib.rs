#![allow(improper_ctypes_definitions)] // Hide the 'not FFI-safe' warnings!

#[macro_use]
extern crate rutie;

use rutie::{Object, RString};

class!(ExampleRustLib);

methods!(
    ExampleRustLib,
    _itself,

    fn pub_hello() -> RString {
        RString::new_utf8(
          "Hello (again) from Rutie example_rust_lib!"
        )
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_example_rust_lib() {
    rutie::Class::new("ExampleRustLib", None).define(|itself| {
        itself.def_self("hello", pub_hello);
    });
}

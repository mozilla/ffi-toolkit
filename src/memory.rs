/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::ffi::{
    CString,
};
use std::os::raw::{
    c_char,
    c_void,
};

/// Creates a function with a given `$name` that releases the memory for a type `$t`.
#[macro_export]
macro_rules! define_destructor (
    ($name:ident, $t:ty) => (
        #[no_mangle]
        pub extern "C" fn $name(obj: *mut $t) {
            let _ = unsafe{ Box::from_raw(obj) };
        }
    )
);

/// Creates a function with a given `$name` that releases the memory
/// for a type `$t` with lifetimes <'a, 'c>.
/// TODO: Move to using `macro_rules` lifetime specifier when it lands in stable
/// This will enable us to specialise `define_destructor` and use repetitions
/// to allow more generic lifetime handling instead of having two functions.
/// https://github.com/rust-lang/rust/issues/34303
/// https://github.com/mozilla/mentat/issues/702
#[macro_export]
macro_rules! define_destructor_with_lifetimes (
    ($name:ident, $t:ty) => (
        #[no_mangle]
        pub extern "C" fn $name<'a, 'c>(obj: *mut $t) {
            let _ = unsafe{ Box::from_raw(obj) };
        }
    )
);

/// destroy function for releasing the memory for boxed `repr(C)` structs.
define_destructor!(destroy, c_void);

#[no_mangle]
pub extern "C" fn destroy_raw_uuid(obj: *mut [u8; 16]) {
    let _ = unsafe{ Box::from_raw(obj) };
}

#[no_mangle]
pub extern "C" fn destroy_c_char(s: *mut c_char) {
    let _ = unsafe { CString::from_raw(s) };
}

#[macro_export]
macro_rules! assert_pointer_not_null {
    ($($e:expr),+ $(,)*) => ($(
        assert!(!$e.is_null(), concat!("Unexpected null pointer: ", stringify!($e)));
    )+);
}

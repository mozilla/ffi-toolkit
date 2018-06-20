/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std;

use std::os::raw::{
    c_char,
    c_void,
};

use string::{
    string_to_c_char,
};


#[repr(C)]
#[derive(Debug)]
pub enum ErrorCode {
    Other,
    AuthenticationError,
}

/// An error struct containing an error code and a description string.
/// #Safety
///
/// Callers are responsible for managing the memory for the return value.
/// A destructor `free_extern_error` is provided for releasing the memory for this
/// pointer type.
#[repr(C)]
#[derive(Debug)]
pub struct ExternError {
    pub code: ErrorCode,
    pub message: *const c_char,
}

/// A C representation of Rust's [Result](std::result::Result).
/// A value of `Ok` results in `ok` containing a raw pointer as a `c_void`
/// and `err` containing a null pointer.
/// A value of `Err` results in `value` containing a null pointer and `err` containing an error struct.
///
/// #Safety
///
/// Callers are responsible for managing the memory for the return value.
/// A destructor `extern_result_destroy` is provided for releasing the memory for this
/// pointer type.
#[repr(C)]
#[derive(Debug)]
pub struct ExternResult {
    pub ok: *const c_void, // We could have used `*const T` instead, but that would have meant creating one `free` function per variant.
    pub err: *const ExternError,
}

impl ExternResult {
    pub fn ok<T>(result: T) -> *mut Self {
        Self::ok_ptr(Box::into_raw(Box::new(result)))
    }

    pub fn ok_ptr<T>(result: *mut T) -> *mut Self {
        Box::into_raw(Box::new(ExternResult {
            ok: result as *const _ as *const c_void,
            err: std::ptr::null_mut(),
        }))
    }

    pub fn ok_null() -> *mut Self {
        Box::into_raw(Box::new(ExternResult {
            ok: std::ptr::null_mut(),
            err: std::ptr::null_mut(),
        }))
    }

    pub fn ok_optional<T>(result: Option<T>) -> *mut Self {
        match result {
            Some(v) => ExternResult::ok(v),
            None => ExternResult::ok_null(),
        }
    }

    pub fn err<S>(code: ErrorCode, msg: S) -> *mut Self
    where S: Into<String> {
        Box::into_raw(Box::new(ExternResult {
            ok: std::ptr::null_mut(),
            err: Box::into_raw(Box::new(ExternError {
                code,
                message: string_to_c_char(msg),
            })),
        }))
    }
}

impl<T, E> From<Result<T, E>> for ExternResult where E: std::error::Error {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => {
                ExternResult {
                    ok: Box::into_raw(Box::new(value)) as *const _ as *const c_void,
                    err: std::ptr::null(),
                }
            },
            Err(e) => {
                ExternResult {
                    ok: std::ptr::null(),
                    err: Box::into_raw(Box::new(ExternError {
                        code: ErrorCode::Other,
                        message: string_to_c_char(e.to_string()),
                    })),
                }
            }
        }
    }
}

define_destructor!(extern_result_destroy, ExternResult);
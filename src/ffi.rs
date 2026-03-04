use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::format_str;

/// Format a GenExpr string.
///
/// Returns a newly-allocated C string on success, or NULL on error.
/// The caller must free the result with `gen_fmt_free`.
#[no_mangle]
pub extern "C" fn gen_fmt_format(
    input: *const c_char,
    skip_idempotence: i32,
    tolerate_parsing_errors: i32,
) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let input_str = match unsafe { CStr::from_ptr(input) }.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    match format_str(input_str, skip_idempotence != 0, tolerate_parsing_errors != 0) {
        Ok(result) => match CString::new(result) {
            Ok(cs) => cs.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a string returned by `gen_fmt_format`.
#[no_mangle]
pub extern "C" fn gen_fmt_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}

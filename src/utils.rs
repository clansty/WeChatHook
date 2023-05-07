
use std::ffi::{CStr};
use std::os::raw::c_char;
use libc::strlen;

pub fn get_pointed_string(s_addr: *const u8) -> String {
    let str_ptr = unsafe { *(s_addr as *const *const c_char) };
    let len = unsafe { strlen(str_ptr) };
    let str_slice = unsafe { std::slice::from_raw_parts(str_ptr as *const u8, len as usize + 1) };
    let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(str_slice) };
    return cstr.to_string_lossy().into_owned();
}

pub fn get_addressed_string(str_ptr: *const i8) -> String {
    let len = unsafe { strlen(str_ptr) };
    let str_slice = unsafe { std::slice::from_raw_parts(str_ptr as *const u8, len as usize + 1) };
    let cstr = unsafe { CStr::from_bytes_with_nul_unchecked(str_slice) };
    return cstr.to_string_lossy().into_owned();
}
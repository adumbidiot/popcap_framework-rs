#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::c_void;

pub const INVALID_HANDLE_VALUE: *mut c_void = -1isize as *mut c_void;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

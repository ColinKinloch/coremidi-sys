#![cfg(any(target_os = "macos", target_os = "ios"))]
#![feature(libc)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

//! Raw C API.

pub extern crate core_foundation_sys;

use core_foundation_sys::*;
use core_foundation_sys::array::*;
use core_foundation_sys::uuid::*;
use core_foundation_sys::runloop::*;
use core_foundation_sys::string::*;
use core_foundation_sys::data::*;
use core_foundation_sys::dictionary::*;
use core_foundation_sys::propertylist::*;

extern crate libc;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

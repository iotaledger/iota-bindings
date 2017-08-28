#![no_std]
#![feature(alloc)]

extern crate alloc;

extern crate iota_mam_bindings;
extern crate iota_bindings;
extern crate iota_bindings_shared as shared;

pub mod ctrits;

pub use iota_mam_bindings::*;
pub use iota_bindings::*;
pub use ctrits::*;


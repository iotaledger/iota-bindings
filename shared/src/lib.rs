#![no_std]
#![feature(alloc)]

#[macro_use]
extern crate std;

extern crate cty;
extern crate alloc;
extern crate iota_trytes as trytes;

pub mod ctrits;
pub use ctrits::*;

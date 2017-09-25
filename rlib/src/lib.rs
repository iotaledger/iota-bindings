#![no_std]
#![feature(alloc)]

extern crate alloc;
extern crate cty;

#[cfg(feature = "mam")]
extern crate iota_mam_bindings;

#[cfg(feature = "iotars")]
extern crate iota_bindings;

extern crate iota_bindings_shared as shared;

pub mod ctrits;

#[cfg(feature = "mam")]
pub use iota_mam_bindings::*;

#[cfg(feature = "iotars")]
pub use iota_bindings::*;

pub use ctrits::*;


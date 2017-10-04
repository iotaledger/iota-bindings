use core;
use alloc::boxed::Box;
use alloc::String;
use alloc::Vec;

use cty::*;
use shared::*;

#[no_mangle]
pub fn iota_ctrits_drop(ctrits: *mut CTrits) {
    let ct = unsafe { Box::from_raw(ctrits) };
    unsafe { Vec::from_raw_parts(ct.data as *mut u8, ct.byte_length, ct.byte_length)};
}

#[no_mangle]
pub fn iota_ctrits_convert(ctrits: &CTrits, to: TritEncoding) -> *const CTrits {
    let out = Box::new(ctrits_convert(ctrits, to));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_ctrits_ctrits_from_trytes(ptr: *const u8, len: usize) -> *const CTrits {
    let slice: &[u8] = unsafe { core::slice::from_raw_parts(ptr, len) };
    let string = String::from_utf8(slice.to_vec()).ok().unwrap();
    let out = Box::new(ctrits_from_trytes(string));

    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_ctrits_ctrits_from_trits(ptr: *const i8, len: usize) -> *const CTrits {
    let slice: &[i8] = unsafe { core::slice::from_raw_parts(ptr, len) };
    let out = Box::new(ctrits_from_trits(slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub fn iota_ctrits_ctrits_from_bytes(trit_len: usize, ptr: *const u8, len: usize) -> *const CTrits {
    let slice: &[u8] = unsafe { core::slice::from_raw_parts(ptr, len) };
    let out = Box::new(ctrits_from_bytes(trit_len, slice.to_vec()));
    Box::into_raw(out)
}

#[no_mangle]
pub unsafe fn iota_ctrits_trits_to_trytes_inplace(ctrits: &mut CTrits) {
    ctrits_trits_to_trytes_inplace(ctrits);
}

#[no_mangle]
pub fn iota_ctrits_ctrits_encoding(ctrits: &CTrits) -> TritEncoding {
    ctrits.encoding.clone()
}

#[no_mangle]
pub fn iota_ctrits_ctrits_length(ctrits: &CTrits) -> usize {
    ctrits.length
}

#[no_mangle]
pub fn iota_ctrits_ctrits_data(ctrits: &CTrits) -> *mut c_void {
    ctrits.data
}

#[no_mangle]
pub fn iota_ctrits_ctrits_byte_length(ctrits: &CTrits) -> usize {
    ctrits.byte_length
}

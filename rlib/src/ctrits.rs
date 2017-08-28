use shared::*;
use alloc::boxed::Box;

#[no_mangle]
pub fn iota_ctrits_drop(ctrits: *mut CTrits) {
    unsafe { Box::from_raw(ctrits) };
}

#[no_mangle]
pub fn iota_ctrits_convert(ctrits: &CTrits, to: TritEncoding) -> *const CTrits {
    let out = Box::new(ctrits_convert(ctrits, to));
    Box::into_raw(out)
}

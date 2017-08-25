use alloc::heap::{Alloc, Layout, AllocErr};
use cty::*;

pub struct StubAlloc;

extern "C" {
    fn unexec_malloc(size: usize) -> *mut c_void;
    fn unexec_realloc(old_ptr: *mut c_void, new_size: usize) -> *mut c_void;
    fn unexec_free(ptr: *mut c_void);
}


unsafe impl<'a>  Alloc for &'a StubAlloc {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let addr = unexec_malloc(layout.size() as usize);
        if addr.is_null() {
            return Err(AllocErr::Exhausted { request: layout });
        }

        assert_eq!(addr as usize & (layout.align() - 1), 0);
        Ok(addr as *mut u8)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        assert_eq!(ptr as usize & (layout.align() - 1), 0);
        unexec_free(ptr as *mut c_void)
    }

    unsafe fn realloc(
        &mut self,
        ptr: *mut u8,
        _layout: Layout,
        new_layout: Layout,
    ) -> Result<*mut u8, AllocErr> {
        let addr = unexec_realloc(ptr as *mut c_void, new_layout.size() as usize);
        if addr.is_null() {
            return Err(AllocErr::Exhausted { request: new_layout });
        }

        assert_eq!(addr as usize & (new_layout.align() - 1), 0);
        Ok(addr as *mut u8)
    }
}

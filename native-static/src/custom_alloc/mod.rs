mod stub_alloc;

pub use self::stub_alloc::StubAlloc;

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    use core::intrinsics;
    unsafe {
        intrinsics::abort();
    }
}

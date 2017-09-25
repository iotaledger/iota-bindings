// This crate uses std so provides the no_std (but depending on an allocator) bindings
// with the default system allocator.
#![cfg_attr(feature = "custom_alloc", no_std)]
#![cfg_attr(feature = "custom_alloc", feature(alloc, allocator_api, global_allocator, lang_items, core_intrinsics))]
#![cfg_attr(feature = "compiler_builtins", feature(compiler_builtins_lib))]
#![cfg_attr(not(feature = "custom_alloc"), crate_type = "dylib")]
#![cfg_attr(feature = "custom_alloc", crate_type = "staticlib")]

#[cfg(feature = "custom_alloc")]
extern crate alloc;

#[cfg(feature = "custom_alloc")]
extern crate cty;

#[cfg(feature = "custom_alloc")]
mod custom_alloc;

#[cfg(feature = "compiler_builtins")]
extern crate compiler_builtins;

#[cfg(feature = "custom_alloc")]
#[global_allocator]
static ALLOCATOR: custom_alloc::StubAlloc = custom_alloc::StubAlloc;


extern crate iota_bindings_rlib as bindings;

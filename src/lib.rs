// We can't use the standard library
#![no_std]
// Used for the `lang` module
#![feature(lang_items)]
// Used for dynamic allocation
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(panic_info_message)]

// use core::alloc;
extern crate alloc;
use core::alloc::Layout;
// Memory-related functions are in the `mem` module
mod mem;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Error de asignación para layout: {:?}", layout);
}
// Set up the global allocator
#[global_allocator]
static ALLOCATOR: mem::KernelAllocator = mem::KernelAllocator::new();

// Re-export any pub extern functions
pub use mem::*;
// Use collections in the alloc crate
pub use alloc::*;

// Used for initializing global variables
#[macro_use]
extern crate lazy_static;
extern crate spin;

// Used for the println!() and print!() macros
#[macro_use]
mod io;

// Used to set up various no_std related things
mod lang;
pub use lang::*;

mod experimental;
pub use experimental::*;

// Entry points
#[no_mangle]
pub extern "C" fn rust_mod_init() -> i32 {
    println!("Panic probability: {}/{}\n", CONFIG.lock().chance, MAX_RAND);
    0
}

#[no_mangle]
pub extern "C" fn rust_mod_exit() {}

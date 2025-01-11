use core::panic::PanicInfo;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[allow(dead_code)]
extern "C" {
    fn abort() -> !;
    fn panic_c();
}

#[panic_handler]
#[no_mangle]
pub fn rust_begin_panic(info: &PanicInfo) -> ! {
    
    if let Some(location) = info.location() {
        println!("Panic ocurred in file '{}' at line '{}'",
                 location.file(),
                 location.line());
    } else {
        println!("Panic occurred but location information is unavailable.");
    }

    if let Some(message) = info.message() {
        println!("{}", message);
    }

    unsafe {
        clean_up_resource();
        abort(); 
    }
}

use core::alloc::Layout;
#[cfg(not(test))]
#[alloc_error_handler]
pub fn alloc_error(_: Layout) -> ! {
    unsafe { abort() }
}

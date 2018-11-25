#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate volatile;
extern crate lazy_static;
extern crate spin;
#[cfg(test)]
extern crate core;

use core::panic::PanicInfo;
mod vga_buffer;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to RustOS");
    println!("Version: {}", "0.1.0");

    loop {}
}

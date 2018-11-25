#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate array_init;
#[cfg(test)]
extern crate core;
extern crate lazy_static;
extern crate spin;
extern crate volatile;

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

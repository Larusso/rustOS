#![no_std]
#![no_main]

extern crate volatile;
extern crate lazy_static;
extern crate spin;

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to RustOS");
    println!("Version: {}", "0.1.0");

    loop {}
}

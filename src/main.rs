#![no_std]
#![no_main]

extern crate volatile;
extern crate lazy_static;
extern crate spin;

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Welcome to RustOS").unwrap();
    write!(vga_buffer::WRITER.lock(), ", version: {}", "0.1.0").unwrap();
    loop {}
}

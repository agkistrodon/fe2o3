#![no_std]

use core::panic::PanicInfo;
pub mod vga;

extern crate rlibc;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga::print_hello();
    loop {}
}

#![no_std]
#![no_main]

// entry point
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}

// panic
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
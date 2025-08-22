use core::iter::Iterator;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

pub fn print_hello() {
    let hello = b"Hello World!";
    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}
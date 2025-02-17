#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello, world!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, c) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = *c;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0x0f;
    //     }
    // }
    vga_buffer::print_something();
    loop {}
}

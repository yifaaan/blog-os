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
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Hello again\n")
        .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1232.234234
    )
    .unwrap();

    loop {}
}

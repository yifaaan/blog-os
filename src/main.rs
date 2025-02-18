#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

static HELLO: &[u8] = b"Hello, world!";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // in Cargo.toml, test-args = ["-device", "isa-debug-exit,iobase=0xf4,iosize=0x04"]
        // 0xf4 is the I/O port for the QEMU ISA debug exit device
        // 0x04 is the size of the I/O port
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// When not running unit tests, panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

/// When running unit tests, panic handler
#[cfg(test)]
#[panic_handler]
fn panic_test(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        serial_print!("{}...\n", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Entry point for unit tests
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // panic!("Some panic message");
    #[cfg(test)]
    test_main();

    loop {}
}

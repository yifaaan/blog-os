#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
// #![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use blog_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

// pub fn test_runner(tests: &[&dyn Fn()]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test();
//         serial_println!("[test did not panic]");
//         exit_qemu(QemuExitCode::Failed);
//     }
//     exit_qemu(QemuExitCode::Success);
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

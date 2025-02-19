#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use blog_os::{exit_qemu, serial_print, serial_println};
use core::panic::PanicInfo;

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(blog_os::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::stack_overflow...\t");

    blog_os::init();
    init_test_idt();

    stack_overflow();
    panic!("Execution continued after stack overflow");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(_info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow();
    volatile::Volatile::new(0).read();
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(blog_os::QemuExitCode::Success);
    loop {}
}

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::{boxed::Box, rc::Rc, string::ToString, vec, vec::Vec};

use blog_os::{
    allocator,
    memory::{self, EmptyFrameAllocator},
    println,
    task::{keyboard::print_keypresses, simple_executor::SimpleExecutor, Task},
};

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::structures::paging::Page;

/// When not running unit tests, panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    blog_os::hlt_loop();
}

/// When running unit tests, panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

entry_point!(kernel_main);

/// Entry point for the kernel
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use crate::memory::BootInfoFrameAllocator;

    use x86_64::VirtAddr;

    println!("Hello, World{}", "!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    unsafe {
        allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();
    }

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(async_task()));
    executor.spawn(Task::new(print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    blog_os::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn async_task() {
    let number = async_number().await;
    println!("number: {}", number);
}

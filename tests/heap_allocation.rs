#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use core::panic::PanicInfo;

use alloc::boxed::Box;
use alloc::{vec, vec::Vec};
use blog_os::allocator::HEAP_SIZE;
use blog_os::{memory::BootInfoFrameAllocator, println};
use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { blog_os::memory::init(phys_mem_offset) };

    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    unsafe {
        blog_os::allocator::init_heap(&mut mapper, &mut frame_allocator).unwrap();
    }

    test_main();

    loop {}
}

#[test_case]
fn simple_allocation() {
    let a = Box::new(1);
    assert_eq!(*a, 1);
    let a = Box::new(vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ]);
    assert_eq!(a.len(), 20);
}

#[test_case]
fn large_vect() {
    let a = (0..10000).collect::<Vec<u32>>();
    assert_eq!(a.len(), 10000);
}

#[test_case]
fn many_boxes() {
    (0..HEAP_SIZE).map(|x| Box::new(x)).count();
}

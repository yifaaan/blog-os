#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{memory::{self, EmptyFrameAllocator}, println};

use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::Page;
use core::panic::PanicInfo;

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
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Hello, World{}", "!");
    blog_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };

    let mut frame_allocator = EmptyFrameAllocator;
    
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_ptr.offset(400).write_volatile(0xf021_f077_f065_f04e);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    blog_os::hlt_loop();
}

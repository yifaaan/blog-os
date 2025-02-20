#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{memory, println};

use bootloader::{entry_point, BootInfo};
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
    let mapper = unsafe {
        memory::init(phys_mem_offset)
    };

    let address = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to the physical address 0
        boot_info.physical_memory_offset,
    ];

    for &addr in address.iter() {
        let virt = VirtAddr::new(addr);
        let phys = mapper.translate_addr(virt);

        println!("{:?}: {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    blog_os::hlt_loop();
}

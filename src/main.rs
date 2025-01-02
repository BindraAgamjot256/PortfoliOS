#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(PortfoliOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::structures::paging::Page;
use x86_64::VirtAddr;
use PortfoliOS::memory::BootInfoFrameAllocator;
use PortfoliOS::vga_buffer::color_code::Color;
use PortfoliOS::vga_buffer::print_colored;
use PortfoliOS::{hlt_loop, memory, println};

/// Entry point for the PortfoliOS kernel.
///
///
/// # Returns
///
/// This function does not return.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to PortfoliOS. I hope you enjoy your stay!");
    PortfoliOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    print_colored(Color::Cyan, format_args!("Hello World!"));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    hlt_loop();
}
entry_point!(kernel_main);

/// Handles panics in non-test configurations.
///
/// This function is called when a panic occurs and prints the panic information.
///
/// # Arguments
///
/// * `info` - Information about the panic.
///
/// # Returns
///
/// This function does not return.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

/// Handles panics in test configurations.
///
/// This function is called when a panic occurs during testing and delegates
/// to the test panic handler.
///
/// # Arguments
///
/// * `info` - Information about the panic.
///
/// # Returns
///
/// This function does not return.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    PortfoliOS::test_panic_handler(info)
}

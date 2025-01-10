#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(PortfoliOS::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::structures::paging::Page;
use x86_64::VirtAddr;
use PortfoliOS::vga_buffer::color_code::Color;
use PortfoliOS::vga_buffer::print_colored;
use PortfoliOS::{hlt_loop, println};

//noinspection ALL
/// Entry point for the PortfoliOS kernel.
///
///
/// # Returns
///
/// This function does not return.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use PortfoliOS::allocator; // new import
    use PortfoliOS::memory::{self, BootInfoFrameAllocator};

    println!("Welcome to PortfoliOS. I hope you enjoy your stay!");
    PortfoliOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_counted);
    println!(
        "reference count is {} now",
        Rc::strong_count(&cloned_reference)
    );

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

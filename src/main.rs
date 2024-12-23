#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(PortfoliOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use PortfoliOS::println;

/// Entry point for the PortfoliOS kernel.
///
///
/// # Returns
///
/// This function does not return.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to PortfoliOS. I hope you enjoy your stay!");

    PortfoliOS::init(); // initialize the IDT

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

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
    loop {}
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

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(PortfoliOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

/// Entry point of the test.
///
/// This function is called on boot and runs the `test_main` function,
/// which executes all the test cases.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

use PortfoliOS::println;

#[test_case]
/// Test case for the `println` function.
///
/// This function tests the `println` macro by printing a test message.
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
/// Panic handler function.
///
/// This function is called on panic and delegates to the test panic handler.
fn panic(info: &PanicInfo) -> ! {
    PortfoliOS::test_panic_handler(info)
}

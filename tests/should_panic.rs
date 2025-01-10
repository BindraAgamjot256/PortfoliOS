#![no_std]
#![no_main]

use core::panic::PanicInfo;
use PortfoliOS::{exit_qemu, serial_print, serial_println, QemuExitCode};

/// Entry point of the test.
///
/// This function is called on boot and runs the `should_fail` function.
/// If the function does not panic, it prints a failure message and exits QEMU with a failure code.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_should_fail();
    serial_println!("[fail]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Function that is expected to fail.
///
/// This function prints a message and asserts that 0 equals 1, which will cause a panic.
fn test_should_fail() {
    serial_print!("should_panic::test_should_fail...\t");
    assert_eq!(0, 1);
}

/// Panic handler function.
///
/// This function is called on panic, prints a success message, and exits QEMU with a success code.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod vga_buffer; // gdt = Global Descriptor Table

/// A trait for testable functions.
///
/// This trait is implemented for any function that can be run as a test.
pub trait Testable {
    /// Runs the test function.
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    /// Runs the test function and prints the result.
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

/// Runs the provided tests and exits QEMU.
///
/// This function is called by the test harness to run all tests.
///
/// # Arguments
///
/// * `tests` - A slice of references to testable functions.
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// Handles panics during testing.
///
/// This function is called when a panic occurs during testing.
///
/// # Arguments
///
/// * `info` - Information about the panic.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`.
///
/// This function is called by the test harness to start the tests.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init(); // new
    test_main();
    loop {}
}

/// Handles panics during testing.
///
/// This function is called when a panic occurs during testing.
///
/// # Arguments
///
/// * `info` - Information about the panic.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// Exit codes for QEMU.
///
/// These codes are used to indicate the result of the tests to QEMU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    /// Indicates that the tests succeeded.
    Success = 0x10,
    /// Indicates that the tests failed.
    Failed = 0x11,
}

/// Exits QEMU with the given exit code.
///
/// This function writes the exit code to the QEMU port to exit QEMU.
///
/// # Arguments
///
/// * `exit_code` - The exit code to write to the QEMU port.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Initializes the interrupts.
///
/// This function sets up the Interrupt Descriptor Table (IDT).
pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

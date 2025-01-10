#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use PortfoliOS::serial_print;

/// Entry point of the kernel.
///
/// This function is called on boot and initializes the GDT and IDT,
/// then triggers a stack overflow to test the double fault handler.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("stack_overflow::test_stack_overflow...\t");

    PortfoliOS::gdt::init();
    init_test_idt();

    // Trigger a stack overflow
    test_stack_overflow();

    panic!("Execution continued after stack overflow");
}

/// Panic handler function.
///
/// This function is called on panic and delegates to the test panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    PortfoliOS::test_panic_handler(info)
}

/// Function that causes a stack overflow by infinite recursion.
///
/// This function calls itself recursively without a base case,
/// causing a stack overflow. The `volatile` read prevents tail call optimization.
#[allow(unconditional_recursion)]
fn test_stack_overflow() {
    test_stack_overflow(); // For each recursion, the return address is pushed
    volatile::Volatile::new(0).read(); // Prevent tail recursion optimizations
}

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    /// Interrupt Descriptor Table (IDT) for the test environment.
    ///
    /// This IDT includes a handler for double faults.
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(PortfoliOS::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

/// Initializes the test IDT.
///
/// This function loads the test IDT into the CPU.
pub fn init_test_idt() {
    TEST_IDT.load();
}

use x86_64::structures::idt::InterruptStackFrame;
use PortfoliOS::{exit_qemu, serial_println, QemuExitCode};

/// Double fault handler for the test environment.
///
/// This function is called on a double fault and prints a success message,
/// then exits QEMU with a success code.
extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

use crate::println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use lazy_static::lazy_static;

lazy_static! {
    /// A lazy static instance of the Interrupt Descriptor Table (IDT).
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

/// Initializes the Interrupt Descriptor Table (IDT).
pub fn init_idt() {
    IDT.load();
}

/// Handler function for breakpoint exceptions.
///
/// This function is called when a breakpoint exception occurs.
/// It prints the exception message and the stack frame.
///
/// # Arguments
///
/// * `stack_frame` - The stack frame at the time of the exception.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Tests if a breakpoint exception is caught.
///
/// This function invokes a breakpoint exception to test if the
/// breakpoint handler is correctly set up.
#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

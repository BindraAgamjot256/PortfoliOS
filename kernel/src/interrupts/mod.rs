///! TODO: create interrupt handlers.
pub mod gdt;
pub mod local_apic;
pub mod io_apic;

use pc_keyboard::DecodedKey;
use spin::lazy::Lazy;
use x86_64::{
    registers::control::Cr2,
    structures::{
        idt::InterruptDescriptorTable,
        paging::{Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB}
    },
    PhysAddr,
    VirtAddr,
    instructions::port::Port,
};
use crate::{framebuffer::{
    ConsoleColor,
    color::ColoredWriting,
    update_cursor,
    global_writer::FRAMEBUFFER_WRITER
}, hlt_loop, println, serial_eprintln, serial_println, print, interrupts::{
    local_apic::LOCAL_APIC,
    gdt::DOUBLE_FAULT_IST_INDEX
}, memory::BootInfoFrameAllocator, serial_print};
use crate::shell::GLOBAL_SHELL;

fn create_idt() -> InterruptDescriptorTable {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
        idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(DOUBLE_FAULT_IST_INDEX);
    }    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt.non_maskable_interrupt.set_handler_fn(nmi_interrupt_handler);
    idt[0x20].set_handler_fn(timer_interrupt_handler);
    idt[0x21].set_handler_fn(spurious_interrupt_handler);
    idt[0x22].set_handler_fn(apic_error_handler);
    idt[0x24].set_handler_fn(keyboard_interrupt_handler);
    idt.general_protection_fault.set_handler_fn(gp_interrupt_handler);
    idt
}

pub static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(create_idt);

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    error_code: u64,
) -> ! {
    serial_eprintln!("\x1b[31mDOUBLE FAULT ERROR CODE: {}\x1b[0m", error_code);
    serial_eprintln!("{:#?}", stack_frame);
    hlt_loop()
}

pub extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    println!("\x1b[31mBREAKPOINT EXCEPTION\x1b[0m");
    println!("{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    err: x86_64::structures::idt::PageFaultErrorCode,
) {
    let fault_addr = Cr2::read(); // Get faulting address

    println!("\x1b[31mPAGE FAULT EXCEPTION\x1b[0m");
    serial_eprintln!("{:#?}", stack_frame);
    serial_eprintln!("{:#?}", err);
    serial_eprintln!("{:#?}", fault_addr.unwrap());
    panic!("{}", "PAGE FAULT EXCEPTION".fg(ConsoleColor::Red).as_mut_str());
}

pub extern "x86-interrupt" fn timer_interrupt_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    unsafe { FRAMEBUFFER_WRITER.force_unlock(); }
    update_cursor();
    let binding = LOCAL_APIC.lock();
    let apic = unsafe { binding.as_ref().unwrap().get_mut() };
    unsafe {
        apic.end_of_interrupt();
    }
}

pub extern "x86-interrupt" fn nmi_interrupt_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    println!("\x1b[31mNMI INTERRUPT\x1b[0m");
    serial_println!("NMI INTERRUPT");
    serial_eprintln!("{:#?}", _stack_frame);
}

pub extern "x86-interrupt" fn spurious_interrupt_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    serial_println!("SPURIOUS INTERRUPT");
    serial_println!("{:#?}", _stack_frame);

    unsafe { LOCAL_APIC.force_unlock(); }
    let binding = LOCAL_APIC.lock();

    let apic = unsafe { binding.as_ref().unwrap().get_mut() };

    unsafe {
        apic.end_of_interrupt();
    }
}

pub extern "x86-interrupt" fn apic_error_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
){
    serial_eprintln!("APIC_ERROR INTERRUPT");
    serial_println!("{:#?}", _stack_frame);
    unsafe { LOCAL_APIC.force_unlock(); }
    let binding = LOCAL_APIC.lock();
    let apic = unsafe { binding.as_ref().unwrap().get_mut() };
    unsafe {
        apic.end_of_interrupt();
    }
}

pub extern "x86-interrupt" fn gp_interrupt_handler(
    stack_frame: x86_64::structures::idt::InterruptStackFrame,
    err_code: u64
) {
    serial_println!("GP INTERRUPT HANDLER");
    serial_eprintln!("{:#?}", stack_frame);
    serial_eprintln!("{:#?}", err_code);
    panic!("\x1b[31mGP INTERRUPT HANDLER\x1b[0m");
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(
    _stack_frame: x86_64::structures::idt::InterruptStackFrame,
) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    let key = crate::keyboard::get_key(scancode);
    if let Some(key) = key {
        match key {
            DecodedKey::Unicode(char) => {
                match char {
                    '\n' => {
                        println!();
                        GLOBAL_SHELL.lock().exec();
                    },
                    '\x08' => {
                        GLOBAL_SHELL.lock().pop();
                        print!("\x08");
                    },
                    _ => {
                        GLOBAL_SHELL.lock().append(char);
                        print!("{}", char);
                    }
                }

            }
            DecodedKey::RawKey(key) => {
                serial_print!("{:#?}  ", key);
            }
        }
    }

    let binding = LOCAL_APIC.lock();
    let apic = unsafe { binding.as_ref().unwrap().get_mut() };
    unsafe {
        apic.end_of_interrupt();
    }
}






/// Identity map the APIC's physical address so it can be accessed virtually.
pub fn map_apic(apic_base: u64, mapper: &mut OffsetPageTable, frame_allocator: &mut BootInfoFrameAllocator) {
    let phys: PhysFrame<Size4KiB> = PhysFrame::containing_address(PhysAddr::new(apic_base));
    let virt = VirtAddr::new(apic_base);
    let page = Page::containing_address(virt);
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    unsafe {
        mapper.map_to(page, phys, flags, frame_allocator)
            .expect("mapping failed")
            .flush();
    }
}
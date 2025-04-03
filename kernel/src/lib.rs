#![feature(abi_x86_interrupt)]
#![no_std]

extern crate alloc;
use acpi::{AcpiTables, InterruptModel};
use bootloader_api::info::PixelFormat;
use core::marker::PhantomData;
use embedded_graphics::pixelcolor::{Bgr888, Gray8, Rgb888};

pub mod allocator;
pub mod framebuffer;
pub mod interrupts;
pub mod memory;
pub mod power;
pub mod serial;
pub mod keyboard;
pub mod shell;

use x86_64::{
    instructions::tables::load_tss,
    instructions::segmentation::Segment,
    registers::segmentation::CS,
    registers::segmentation::{DS, ES, SS}
};
use crate::{
    framebuffer::ConsoleColor,
    framebuffer::color::ColoredWriting,
    interrupts::{local_apic, map_apic},
    framebuffer::{boot_animation, boot_finished, init_framebuffer_writer},
    interrupts::gdt::GDT,
    interrupts::{IDT},
    power::{KernelAcpiHandler, FADT_ADDR},
    interrupts::io_apic::{init_globally_available_io_apic, IO_APIC},
    interrupts::local_apic::{LOCAL_APIC}
};


/// Halt the CPU until the next interrupt occurs.
///
/// ### returns:
/// - `!`: Never returns, as if the OS were to return, then there would be nothing to hand over
/// control to...
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Kernel Initialisation routine, called immediately after boot, by the
/// `kernel_main()` function, in the main.rs.
///
/// ### params:
/// - `boot_info`: A mutable reference to the bootloader's BootInfo struct.
///
/// ### returns:
/// - `()`: Nothing/Void.
pub fn init(boot_info: &'static mut bootloader_api::BootInfo) -> () {
    // if the framebuffer, exists, then init it, else, panic.
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();  // Get the framebuffer info
        let buffer = framebuffer.buffer_mut(); // Get the framebuffer buffer
        let pixel_fmt = info.pixel_format; // Get the pixel format


        // init the frambuffer writer, based on the pixel format.
        match pixel_fmt {
            PixelFormat::Rgb => {
                init_framebuffer_writer::<Rgb888>(buffer, info, PhantomData);
            }
            PixelFormat::Bgr => {
                init_framebuffer_writer::<Bgr888>(buffer, info, PhantomData);
            }
            PixelFormat::U8 => {
                init_framebuffer_writer::<Gray8>(buffer, info, PhantomData);
            }
            _ => {
                init_framebuffer_writer::<Gray8>(buffer, info, PhantomData);
            }
        }
        // play boot animation
        boot_animation();
    } else {
        // TODO: use serial_println!() here, instead of println
        panic!("No framebuffer found");
    }

    GDT.0.load();

    serial_println!("GDT/IDT init time...");
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        // Set DS, ES, and SS to your data segment selector.
        // You need to add a data segment in your GDT for this.
        DS::set_reg(GDT.1.data_selector);
        ES::set_reg(GDT.1.data_selector);
        SS::set_reg(GDT.1.data_selector);
        load_tss(GDT.1.tss_selector);
        //load_tss(GDT.tss_selector);
    }// Load the Global Descriptor Table

    serial_println!("GDT loaded.");
    IDT.load(); // Load the Interrupt Descriptor Table
    serial_println!("IDT loaded.");
    // init the heap.
    let phys_mem_offset = x86_64::VirtAddr::new(
        boot_info
            .physical_memory_offset
            .into_option()
            .expect("Physical memory offset not found"),
    );
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_regions) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let phys_offset = boot_info.physical_memory_offset.into_option().unwrap();

    let acpi_handler = KernelAcpiHandler::new(phys_offset); // Create a new ACPI handler

    // Parse the ACPI tables
    let acpi_tables = unsafe {
        AcpiTables::from_rsdp(
            acpi_handler,
            boot_info
                .rsdp_addr
                .into_option()
                .expect("RSDP address not found") as usize,
        )
        .expect("Failed to parse ACPI tables")
    };

    let platform_info = acpi_tables.platform_info().unwrap(); // Get the platform info

    let interrupt_model = platform_info.interrupt_model;

    let fadt_addr = acpi_tables
        .find_table::<acpi::fadt::Fadt>()
        .map(|fadt| &fadt as *const _ as usize)
        .expect("Why the Fuck does this not exist????");

    *FADT_ADDR.lock() = Some(fadt_addr);

    serial_println!("{:#?}", interrupt_model);

    let lapic_base:usize = match interrupt_model.clone() {
        InterruptModel::Apic(apic) => {
            apic.local_apic_address as usize

        }
        _ => { 0xFEE00000 }
    };

    map_apic(lapic_base as u64, &mut mapper, &mut frame_allocator);
    unsafe { local_apic::init_globally_available_local_apic(lapic_base); }
    serial_println!("Local APIC Base Address: {:#x}", lapic_base);


    let local_apic_binding = LOCAL_APIC.lock();
    let local_apic = unsafe { local_apic_binding.as_ref().unwrap().get_mut() };
    unsafe{
        local_apic.enable();
    }
    serial_println!("Local APIC Initialized");

    let io_apic_base = match interrupt_model.clone() {
        InterruptModel::Apic(apic) => {
            apic.io_apics.iter().as_slice()[0].address
        }
        _ => {0xFEC00000}
    };
    map_apic(io_apic_base as u64, &mut mapper, &mut frame_allocator);
    unsafe { init_globally_available_io_apic(io_apic_base as u64);}
    serial_println!("IO APIC Base Address: {:#x}", io_apic_base);

    let io_apic_binding = IO_APIC.lock();
    let io_apic = io_apic_binding.as_ref().unwrap().get_mut();
    unsafe {
        io_apic.init(0x23);
        io_apic.enable_irq(1);
    }

    serial_println!("{}","Enabling Interrupts...".fg(ConsoleColor::BrightGreen));
    boot_finished();
}

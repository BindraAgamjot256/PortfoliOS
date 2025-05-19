#![no_std]
#![no_main]

extern crate alloc;
use bootloader_api::config::Mapping;
use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use core::panic::PanicInfo;
use kernel::framebuffer::{color::ColoredWriting, ConsoleColor};
use kernel::{hlt_loop, println, serial_println};
use kernel::shell::GLOBAL_SHELL;

static CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

/// The Kernel Main function. Called by the bootloader, indirectly
/// through the [entry_point](entry_point) macro.
///
/// ```
/// entry_point!(kernel_main,...)
/// // expands to:
/// extern "C" fn _start(boot_info: &'static mut BootInfo) -> ! {
///     kernel_main(boot_info)
/// }
/// ```
pub fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init(boot_info);

    x86_64::instructions::interrupts::enable();
    serial_println!("{}","Enabled Interrupts...".fg(ConsoleColor::BrightGreen));

    // We can continue with our main execution flow
    println!("Hello, World!");
    println!("\x1b[32mHello, World!\x1b[0m");
    println!("{}", "Hi there!".fg(ConsoleColor::BrightCyan));
    GLOBAL_SHELL.lock().init();

    hlt_loop();
}

entry_point!(kernel_main, config = &CONFIG);

#[panic_handler]
#[allow(unused_unsafe)]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    unsafe {
        serial_println!("SHUTTING DOWN...");
    }
    hlt_loop();
}

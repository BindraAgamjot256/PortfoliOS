use crate::framebuffer::color::ColoredWriting;
use crate::framebuffer::ConsoleColor;
use core::fmt;
use spin::Mutex;
use uart_16550::SerialPort;

/// Global serial port instance (using the standard I/O port 0x3F8).
pub static SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

/// Initialize the serial port—call this early in your kernel’s setup.
pub fn init() {
    SERIAL1.lock().init();
}

/// Internal helper: write formatted arguments to the serial port.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).unwrap();
}

/// Internal helper: write error-formatted arguments to the serial port.
/// This example prefixes error messages with "ERROR: " for clarity.
#[doc(hidden)]
pub fn _eprint(args: fmt::Arguments) {
    use core::fmt::Write;
    let mut serial = SERIAL1.lock();
    write!(serial, "{}", "ERROR: ".fg(ConsoleColor::Red)).unwrap();
    serial.write_fmt(args).unwrap();
}

/// Macro for printing to the serial port.
/// Usage: `serial_print!("Hello {}!", "world");`
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*))
    };
}

/// Macro for printing to the serial port with a newline at the end.
/// Usage: `serial_println!("Hello {}!", "world");`
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

/// Macro for printing error messages to the serial port.
/// Usage: `serial_eprint!("An error occurred: {}", err);`
#[macro_export]
macro_rules! serial_eprint {
    ($($arg:tt)*) => {
        $crate::serial::_eprint(format_args!($($arg)*))
    };
}

/// Macro for printing error messages with a newline.
/// Usage: `serial_eprintln!("An error occurred: {}", err);`
#[macro_export]
macro_rules! serial_eprintln {
    () => ($crate::serial_eprint!("\n"));
    ($fmt:expr) => ($crate::serial_eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_eprint!(concat!($fmt, "\n"), $($arg)*));
}

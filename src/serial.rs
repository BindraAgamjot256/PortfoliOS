use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    /// A lazy static instance of a serial port wrapped in a mutex.
    ///
    /// This instance is used to ensure safe access to the serial port
    /// from multiple threads.
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// Prints formatted arguments to the serial interface.
///
/// This function is used internally by the `serial_print!` and `serial_println!`
/// macros to send formatted strings to the serial port.
///
/// # Arguments
///
/// * `args` - The formatted arguments to print.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

/// Prints to the host through the serial interface.
///
/// This macro sends formatted strings to the serial port using the `_print`
/// function.
///
/// # Examples
///
/// ```
/// PortfoliOS::serial_print!("Hello, world!");
/// ```
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
///
/// This macro sends formatted strings to the serial port using the `_print`
/// function and appends a newline character.
///
/// # Examples
///
/// ```
/// PortfoliOS::serial_println!("Hello, world!");
/// ```
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

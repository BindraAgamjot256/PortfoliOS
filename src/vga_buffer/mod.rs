use core::fmt;

pub mod color_code;
pub mod writing;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        // new
        writing::WRITER.lock().write_fmt(args).unwrap();
    });
}

pub fn print_colored(color: color_code::Color, args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = writing::WRITER.lock();
        let color_code = color_code::ColorCode::new(color, color_code::Color::Black);
        writer.set_color(color_code);
        writer.write_fmt(args).unwrap();
        writer.set_color(color_code::ColorCode::new(
            color_code::Color::Yellow,
            color_code::Color::Black,
        ));
    });
}

pub fn println_colored(color: color_code::Color, args: fmt::Arguments) {
    print_colored(color, format_args!("{}\n", args));
}

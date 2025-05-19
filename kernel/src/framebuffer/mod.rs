pub mod animation;
pub mod color;
pub mod global_writer;
pub mod writer;

// Re-export commonly used functions and types for easier access
pub use animation::{boot_animation, boot_finished};
pub use color::ConsoleColor;
pub use global_writer::{_print, print_fmt, init_framebuffer_writer, update_cursor};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::framebuffer::_print(format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)*)
    };
}

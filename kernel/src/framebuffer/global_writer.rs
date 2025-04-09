use bootloader_api::info::FrameBufferInfo;
use core::fmt;
use core::marker::PhantomData;
use spin::Lazy;
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;
use crate::framebuffer::color::ConsoleColor;
use crate::framebuffer::writer::{FrameBufferWriter, PixelConversion};

/// A lazily initialized, spin‑mutex–protected global framebuffer writer.
/// The writer is wrapped in an Option so that it can be set once at initialization.
pub(crate) static FRAMEBUFFER_WRITER: Lazy<Mutex<Option<FrameBufferWriter>>> = Lazy::new(|| Mutex::new(None));

pub fn init_framebuffer_writer<P: PixelConversion>(
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    pixel_type: PhantomData<P>,
) {
    *FRAMEBUFFER_WRITER.lock() = Some(FrameBufferWriter::new(framebuffer, info, pixel_type));
}

/// Provides access to the global framebuffer writer. The supplied closure is executed
/// with a mutable reference to the writer. Panics if the writer has not been initialized.
pub fn with_writer<R>(f: impl FnOnce(&mut FrameBufferWriter) -> R) -> R {
    without_interrupts(|| {
        let mut guard = FRAMEBUFFER_WRITER.lock();
        let writer = guard.as_mut().expect("Framebuffer writer not initialized");
        f(writer)
    })
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    with_writer(|writer| {
        use core::fmt::Write;
        writer.write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
pub fn _print_fmt(args: fmt::Arguments, color_fg: ConsoleColor, color_bg: ConsoleColor) {
    with_writer(|writer| {
        use core::fmt::Write;
        writer.set_text_color(color_fg);
        writer.set_background_color(color_bg);
        writer.write_fmt(args).unwrap();
        writer.set_text_color(ConsoleColor::BrightWhite);
        writer.set_background_color(ConsoleColor::Black);
    });
}

/// Update the cursor Status(on or off), and position.
pub fn update_cursor() {
    with_writer(|writer| {
        writer.update_cursor();
    })
}

/// Clear the screen.
pub fn clear_screen() {
    with_writer(|writer| {
        let _ = writer.clear_screen().unwrap();
    })
}
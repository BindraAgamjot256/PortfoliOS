use alloc::string::ToString;
use bootloader_api::info::FrameBufferInfo;
use core::convert::Infallible;
use core::fmt;
use core::fmt::Write;
use core::marker::PhantomData;
use core::str;
use embedded_graphics::mono_font::{ascii::FONT_10X20, MonoTextStyle};
use embedded_graphics::text::Text;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use embedded_graphics::pixelcolor::Gray8;

use crate::framebuffer::color::{ansi_color_to_console_color, ConsoleColor};

/// Trait to convert an `Rgb888` color to the appropriate pixel format and write it to the framebuffer.
///
/// This trait abstracts over the pixel format conversion, enabling support for different color orders
/// or even grayscale conversion.
pub trait PixelConversion {
    /// Writes a pixel at the specified index in the framebuffer with the given color.
    ///
    /// - `framebuffer`: The mutable slice representing the framebuffer.
    /// - `pixel_index`: The index of the pixel to update.
    /// - `color`: The color to write (in Rgb888 format).
    /// - `info`: FrameBufferInfo containing metadata like bytes per pixel.
    fn write_pixel(
        framebuffer: &mut [u8],
        pixel_index: usize,
        color: Rgb888,
        info: &FrameBufferInfo,
    );
}

/// Implementation for the `Rgb888` pixel format.
/// Writes color in R, G, B order.
impl PixelConversion for Rgb888 {
    fn write_pixel(
        framebuffer: &mut [u8],
        pixel_index: usize,
        color: Rgb888,
        info: &FrameBufferInfo,
    ) {
        let byte_offset = pixel_index * info.bytes_per_pixel;
        // Write bytes in RGB order.
        framebuffer[byte_offset] = color.r();
        framebuffer[byte_offset + 1] = color.g();
        framebuffer[byte_offset + 2] = color.b();
    }
}

/// Implementation for the `Bgr888` pixel format from embedded-graphics.
/// Writes color in B, G, R order.
use embedded_graphics::pixelcolor::Bgr888;
use crate::shell::GLOBAL_SHELL;

impl PixelConversion for Bgr888 {
    fn write_pixel(
        framebuffer: &mut [u8],
        pixel_index: usize,
        color: Rgb888,
        info: &FrameBufferInfo,
    ) {
        let byte_offset = pixel_index * info.bytes_per_pixel;
        // Write bytes in BGR order.
        framebuffer[byte_offset] = color.b();
        framebuffer[byte_offset + 1] = color.g();
        framebuffer[byte_offset + 2] = color.r();
    }
}

/// Implementation for grayscale conversion using `Gray8`.
/// Converts the Rgb888 color into grayscale by averaging the channels.
impl PixelConversion for Gray8 {
    fn write_pixel(
        framebuffer: &mut [u8],
        pixel_index: usize,
        color: Rgb888,
        info: &FrameBufferInfo,
    ) {
        let byte_offset = pixel_index * info.bytes_per_pixel;
        // Calculate average for grayscale conversion.
        let gray = ((color.r() as u16 + color.g() as u16 + color.b() as u16) / 3) as u8;
        // For grayscale, only one byte is used per pixel.
        framebuffer[byte_offset] = gray;
    }
}

/// A framebuffer writer that implements the embedded‑graphics `DrawTarget` trait,
/// enabling it to be used as a drawing surface for text and other graphics.
///
/// This struct holds the framebuffer, rendering settings, and a cursor for text output.
pub struct FrameBufferWriter {
    /// Mutable reference to the framebuffer memory.
    pub framebuffer: &'static mut [u8],
    /// Information about the framebuffer (e.g., dimensions, bytes per pixel, stride).
    pub info: FrameBufferInfo,
    /// Current x-position (in pixels) of the cursor.
    pub cursor_x: usize,
    /// Current y-position (in pixels) of the cursor.
    pub cursor_y: usize,
    /// Padding in pixels around the screen.
    pub padding: usize,
    /// Width of the font being used.
    pub font_width: usize,
    /// Height of the font being used.
    pub font_height: usize,
    /// Vertical spacing between lines.
    pub line_spacing: usize,
    /// Current text color.
    pub text_color: ConsoleColor,
    /// Current background color.
    pub background_color: ConsoleColor,
    /// Whether the cursor is currently visible (for blinking).
    pub cursor_visible: bool,
    /// Timer used to control cursor blinking.
    pub cursor_blink_timer: usize,
    /// Last recorded x-position of the cursor (used to erase previous cursor drawing).
    pub last_cursor_x: usize,
    /// Last recorded y-position of the cursor.
    pub last_cursor_y: usize,
    /// Function pointer for converting an `Rgb888` color into the framebuffer's pixel format.
    pixel_converter: fn(&mut [u8], usize, Rgb888, &FrameBufferInfo),
}

impl FrameBufferWriter {
    /// Creates a new `FrameBufferWriter` with the provided framebuffer, framebuffer info, and a phantom type
    /// that selects the pixel conversion function.
    ///
    /// # Type Parameters
    ///
    /// - `P`: A type that implements `PixelConversion`. This determines how colors are written to the framebuffer.
    ///
    /// # Parameters
    ///
    /// - `framebuffer`: Mutable reference to the framebuffer memory.
    /// - `info`: FrameBufferInfo with details like dimensions and bytes per pixel.
    /// - `_pixel`: PhantomData to specify the pixel format type.
    ///
    /// # Returns
    ///
    /// A fully initialized `FrameBufferWriter` with a cleared screen.
    pub fn new<P>(
        framebuffer: &'static mut [u8],
        info: FrameBufferInfo,
        _pixel: PhantomData<P>,
    ) -> Self
    where
        P: PixelConversion,
    {
        // Define constants for padding and font size.
        let padding = 10; // Padding around the screen in pixels.
        let font_width = 10; // Width of FONT_10X20.
        let font_height = 21; // Height of FONT_10X20.
        let line_spacing = 23; // Vertical gap between lines.

        // Capture the pixel conversion function from the generic type.
        let pixel_converter = P::write_pixel;

        // Initialize the writer with starting values.
        let mut writer = Self {
            framebuffer,
            info,
            cursor_x: padding,
            cursor_y: padding + font_height, // Start position is first line.
            padding,
            font_width,
            font_height,
            line_spacing,
            text_color: ConsoleColor::BrightWhite,
            background_color: ConsoleColor::Black,
            cursor_visible: true,
            cursor_blink_timer: 0,
            last_cursor_x: padding,
            last_cursor_y: padding + font_height,
            pixel_converter,
        };
        // Clear the framebuffer using the background color.
        writer.clear(writer.background_color.to_rgb888()).unwrap();
        writer
    }

    /// Draws a string of text on the framebuffer with automatic line wrapping.
    ///
    /// This method erases the cursor, draws the text character by character
    /// wrapping to a new line if the text exceeds the screen width, and then redraws the cursor.
    ///
    /// # Parameters
    ///
    /// - `text`: The text string to display.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating success or failure of the text drawing.
    pub fn draw_text(&mut self, text: &str) -> fmt::Result {
        self.draw_wrapped_text(text)
    }

    /// Draws text while supporting ANSI escape sequences for color changes with line wrapping.
    ///
    /// The method parses the text for ANSI escape sequences (like color changes and backspaces),
    /// splits text into segments, updates colors accordingly, and then calls `draw_wrapped_text` for each segment.
    ///
    /// # Parameters
    ///
    /// - `text`: The text string to process and display.
    ///
    /// # Returns
    ///
    /// A `fmt::Result` indicating whether the operation was successful.
    pub fn draw_text_ansi(&mut self, text: &str) -> fmt::Result {
        // Buffer to accumulate a segment of text without escape sequences.
        let mut current_segment_buf = [0u8; 256];
        let mut current_segment_len = 0;
        let mut chars = text.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // On encountering an escape character, flush the current text segment.
                if current_segment_len > 0 {
                    let seg = str::from_utf8(&current_segment_buf[..current_segment_len])
                        .unwrap();
                    self.draw_wrapped_text(seg)?;
                    current_segment_len = 0;
                }
                // Process ANSI escape sequence if the next character is '['.
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '[' {
                        chars.next(); // Consume '['.
                        let mut code_buf = [0u8; 16];
                        let mut code_len = 0;
                        // Collect characters until the terminating 'm' is found.
                        while let Some(&c) = chars.peek() {
                            if c == 'm' {
                                chars.next(); // Consume 'm'
                                break;
                            } else {
                                if code_len < code_buf.len() {
                                    code_buf[code_len] = c as u8;
                                    code_len += 1;
                                }
                                chars.next();
                            }
                        }
                        // Convert collected codes into a string.
                        let code_str = str::from_utf8(&code_buf[..code_len]).unwrap();
                        // Split the codes and adjust colors accordingly.
                        for code in code_str.split(';') {
                            if let Ok(num) = code.parse::<u8>() {
                                // Code 0 resets colors.
                                if num == 0 {
                                    self.set_text_color(ConsoleColor::BrightWhite);
                                    self.set_background_color(ConsoleColor::Black);
                                // Text colors (foreground).
                                } else if (30..=37).contains(&num) || (90..=97).contains(&num) {
                                    if let Some(color) = ansi_color_to_console_color(num, false) {
                                        self.set_text_color(color);
                                    }
                                // Background colors.
                                } else if (40..=47).contains(&num) || (100..=107).contains(&num) {
                                    if let Some(color) = ansi_color_to_console_color(num, true) {
                                        self.set_background_color(color);
                                    }
                                }
                            }
                        }
                    }
                }
            } else if ch == '\n' {
                // If a newline is encountered, flush current segment and move to a new line.
                if current_segment_len > 0 {
                    let seg = str::from_utf8(&current_segment_buf[..current_segment_len])
                        .unwrap();
                    self.draw_wrapped_text(seg)?;
                    current_segment_len = 0;
                }
                self.new_line();
            } else if ch == '\x08' {
                // Handle backspace: erase cursor and move back one character if possible.
                self.erase_cursor();
                // len can be 0 also, so the second check is necessary. *10 occurs because, the font is 10 pixels wide.
                if self.cursor_x > self.padding && self.cursor_x > (GLOBAL_SHELL.lock().len) {
                    self.cursor_x -= self.font_width;
                    self.write_char(' ').expect("If this happens, it is the end of the world.");
                    self.erase_cursor();
                    self.cursor_x -= self.font_width;
                }
                self.update_cursor();
            } else {
                // Accumulate regular character into the segment buffer.
                if current_segment_len < current_segment_buf.len() {
                    current_segment_buf[current_segment_len] = ch as u8;
                    current_segment_len += 1;
                }
            }
        }
        // Draw any remaining text segment.
        if current_segment_len > 0 {
            let seg = str::from_utf8(&current_segment_buf[..current_segment_len]).unwrap();
            self.draw_wrapped_text(seg)?;
        }
        Ok(())
    }

    /// Draws text with automatic line wrapping.
    ///
    /// This method processes the provided text character by character, and if adding another character would exceed the
    /// available width (screen width minus padding), it starts a new line. Newline characters in the text are also respected.
    pub fn draw_wrapped_text(&mut self, text: &str) -> fmt::Result {
        self.erase_cursor();
        for ch in text.chars() {
            if ch == '\n' {
                self.new_line();
                continue;
            }
            let max_width = self.info.width - (self.padding * 2);
            if self.cursor_x + self.font_width > max_width {
                self.new_line();
            }
            let position = Point::new(self.cursor_x as i32, self.cursor_y as i32);
            let style = MonoTextStyle::new(&FONT_10X20, self.text_color.to_rgb888());
            let s = ch.to_string();
            Text::new(&s, position, style)
                .draw(self)
                .map_err(|_| fmt::Error)?;
            self.cursor_x += self.font_width;
        }
        self.draw_cursor();
        Ok(())
    }

    /// Toggles the cursor's visibility for blinking.
    ///
    /// If the cursor becomes visible, it is drawn; otherwise, it is erased.
    pub fn update_cursor(&mut self) {
        self.cursor_visible = !self.cursor_visible;
        if self.cursor_visible {
            self.draw_cursor();
        } else {
            self.erase_cursor();
        }
    }

    /// Erases the cursor from its current position by drawing a rectangle with the background color.
    fn erase_cursor(&mut self) {
        let offset = 3; // Number of pixels to push the cursor down
        let cursor_rect = Rectangle::new(
            Point::new(self.cursor_x as i32, (self.cursor_y - self.font_height + offset) as i32),
            Size::new(self.font_width as u32, self.font_height as u32),
        );
        let bg_style = PrimitiveStyle::with_fill(self.background_color.to_rgb888());
        cursor_rect.into_styled(bg_style).draw(self).unwrap();
    }

    /// Draws the cursor at the current position using the text color.
    ///
    /// This method saves the current cursor position to allow for later erasing.
    pub fn draw_cursor(&mut self) {
        let offset = 3; // Number of pixels to push the cursor down
        self.last_cursor_x = self.cursor_x;
        self.last_cursor_y = self.cursor_y;
        let cursor_rect = Rectangle::new(
            Point::new(self.cursor_x as i32, (self.cursor_y - self.font_height + offset) as i32),
            Size::new(self.font_width as u32, self.font_height as u32),
        );
        let cursor_style = PrimitiveStyle::with_fill(self.text_color.to_rgb888());
        cursor_rect.into_styled(cursor_style).draw(self).unwrap();
    }

    /// Scrolls the screen content up by one text line (using `line_spacing` pixels),
    /// erasing the bottom-most area.
    fn scroll_up(&mut self) -> Result<(), <Self as DrawTarget>::Error> {
        let bytes_per_row = self.info.stride * self.info.bytes_per_pixel;
        let total_rows = self.info.height;
        let scroll_pixels = self.line_spacing;
        let total_bytes = total_rows * bytes_per_row;

        // Move the framebuffer content up by `scroll_pixels` rows.
        self.framebuffer.copy_within(scroll_pixels * bytes_per_row..total_bytes, 0);

        // Clear the newly exposed area (bottom `scroll_pixels` rows).
        for y in (total_rows - scroll_pixels)..total_rows {
            for x in 0..self.info.width {
                let pixel_index = y * self.info.stride + x;
                (self.pixel_converter)(
                    self.framebuffer,
                    pixel_index,
                    self.background_color.to_rgb888(),
                    &self.info,
                );
            }
        }
        Ok(())
    }

    /// Scrolls the screen content down by one text line (using `line_spacing` pixels),
    /// erasing the top-most area.
    ///
    /// This method shifts all current content downward and clears the newly exposed top rows.
    pub fn scroll_down(&mut self) -> Result<(), <Self as DrawTarget>::Error> {
        let bytes_per_row = self.info.stride * self.info.bytes_per_pixel;
        let total_rows = self.info.height;
        let scroll_pixels = self.line_spacing;
        // Move the framebuffer content downward.
        // We iterate in reverse to avoid overwriting data that hasn't been moved yet.
        for y in (0..total_rows - scroll_pixels).rev() {
            let src_range = y * bytes_per_row..y * bytes_per_row + bytes_per_row;
            let dst_start = (y + scroll_pixels) * bytes_per_row;
            self.framebuffer.copy_within(src_range, dst_start);
        }
        // Clear the top `scroll_pixels` rows.
        for y in 0..scroll_pixels {
            for x in 0..self.info.width {
                let pixel_index = y * self.info.stride + x;
                (self.pixel_converter)(
                    self.framebuffer,
                    pixel_index,
                    self.background_color.to_rgb888(),
                    &self.info,
                );
            }
        }
        Ok(())
    }

    /// Moves the cursor to the next line.
    ///
    /// The cursor is reset to the left padding and moved down by the line spacing.
    /// If the cursor exceeds the height of the framebuffer, the screen is scrolled up by one text line.
    fn new_line(&mut self) {
        self.erase_cursor();
        self.cursor_x = self.padding;
        self.cursor_y += self.line_spacing;
        let max_height = self.info.height - self.padding;
        if self.cursor_y > max_height {
            // Scroll the framebuffer up by one text line.
            self.scroll_up().unwrap();
            // Adjust the cursor back one line.
            self.cursor_y -= self.line_spacing;
        }
    }

    /// Sets the cursor position based on column (x) and row (y).
    ///
    /// The positions are adjusted with the configured padding and font dimensions.
    ///
    /// # Parameters
    ///
    /// - `x`: Column number (multiplied by font width).
    /// - `y`: Row number (multiplied by line spacing and adjusted by font height).
    pub fn set_cursor_position(&mut self, x: usize, y: usize) {
        self.erase_cursor();
        self.cursor_x = self.padding + (x * self.font_width);
        self.cursor_y = self.padding + (y * self.line_spacing) + self.font_height;
        self.draw_cursor();
    }

    /// Sets the text color used for drawing text.
    ///
    /// # Parameters
    ///
    /// - `color`: A `ConsoleColor` enum representing the new text color.
    pub fn set_text_color(&mut self, color: ConsoleColor) {
        self.text_color = color;
    }

    /// Sets the background color used when clearing or erasing parts of the screen.
    ///
    /// # Parameters
    ///
    /// - `color`: A [ConsoleColor] enum representing the new background color.
    pub fn set_background_color(&mut self, color: ConsoleColor) {
        self.background_color = color;
    }


    /// Clears the entire framebuffer with the current background color.
    ///
    /// # Returns
    ///
    /// - [Result<(), Infallible>] indicating success or failure.
    pub fn clear_screen(&mut self) -> Result<(), Infallible>{
        self.cursor_x  =10;
        self.cursor_y = 31;
        self.clear(ConsoleColor::Black.to_rgb888())
    }
}

/// Implementation of the `OriginDimensions` trait for `FrameBufferWriter` to provide its dimensions.
impl OriginDimensions for FrameBufferWriter {
    fn size(&self) -> Size {
        Size::new(self.info.width as u32, self.info.height as u32)
    }
}

/// Implementation of the `DrawTarget` trait from embedded-graphics,
/// which allows drawing pixels, clearing the screen, and other graphics operations.
impl DrawTarget for FrameBufferWriter {
    type Color = Rgb888;
    type Error = Infallible;

    /// Draws an iterator of pixels to the framebuffer.
    ///
    /// Each pixel is written using the selected pixel conversion function.
    ///
    /// # Parameters
    ///
    /// - `pixels`: An iterator over `Pixel<Rgb888>` items containing coordinates and colors.
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            // Only draw if the coordinates are within bounds.
            if coord.x >= 0 && coord.y >= 0 {
                let x = coord.x as usize;
                let y = coord.y as usize;
                if x < self.info.width && y < self.info.height {
                    let pixel_index = y * self.info.stride + x;
                    (self.pixel_converter)(self.framebuffer, pixel_index, color, &self.info);
                }
            }
        }
        Ok(())
    }

    /// Clears the entire framebuffer with the specified color.
    ///
    /// # Parameters
    ///
    /// - `color`: The color to fill the screen with.
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                let pixel_index = y * self.info.stride + x;
                (self.pixel_converter)(self.framebuffer, pixel_index, color, &self.info);
            }
        }
        Ok(())
    }
}

/// Implementation of the `fmt::Write` trait, allowing the `FrameBufferWriter` to be used
/// as a target for formatted strings (e.g., with the `write!` macro).
impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Use the ANSI-aware text drawing method.
        self.draw_text_ansi(s)
    }
}

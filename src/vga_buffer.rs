use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts;

/// Represents the color codes for the VGA text mode.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

/// Represents a color code combining foreground and background colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    /// Creates a new `ColorCode` with the given foreground and background colors.
    ///
    /// # Arguments
    ///
    /// * `foreground` - The foreground color.
    /// * `background` - The background color.
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

/// Represents a character on the screen with an ASCII value and a color code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

/// The height of the VGA text buffer.
const BUFFER_HEIGHT: usize = 25;
/// The width of the VGA text buffer.
const BUFFER_WIDTH: usize = 80;

/// Represents the VGA text buffer.
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// A writer type that allows writing to the VGA text buffer.
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes a single byte to the VGA text buffer.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte to write.
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// Writes a string to the VGA text buffer.
    ///
    /// # Arguments
    ///
    /// * `string` - The string to write.
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Moves the cursor to a new line.
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row in the VGA text buffer.
    ///
    /// # Arguments
    ///
    /// * `row` - The row to clear.
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    /// Writes a string to the VGA text buffer.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to write.
    ///
    /// # Returns
    ///
    /// Returns `fmt::Result` indicating success or failure.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    /// A lazy static instance of the `Writer` wrapped in a mutex.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

/// reimplement the `print!` and `println!` macros to use the VGA text buffer
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints formatted arguments to the VGA text buffer.
///
/// This function is used internally by the `print!` and `println!` macros to send formatted strings to the VGA text buffer.
///
/// # Arguments
///
/// * `args` - The formatted arguments to print.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        // new
        WRITER.lock().write_fmt(args).unwrap();
    });
}

/// A simple test case for the `println!` macro.
#[test_case]
fn test_println_simple() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", "GG").expect("writeln failed");
    });
}

/// A test case for the `println!` macro with many lines.
#[test_case]
fn test_println_many() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    for _ in 0..200 {
        interrupts::without_interrupts(|| {
            let mut writer = WRITER.lock();
            writeln!(writer, "\n{}", "GG").expect("writeln failed");
        });
    }
}

/// A test case for the `println!` macro to check the output.
#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}

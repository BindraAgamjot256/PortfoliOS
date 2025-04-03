use crate::framebuffer::global_writer::with_writer;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::Image;
use embedded_graphics::Drawable;
use tinybmp::Bmp;

/// Boot animation: Draws a simple static image as a splash screen and plays a startup sound.
/// In this version, we draw a filled rectangle (as a backdrop) with a circle in the middle.
///
pub fn boot_animation() {
    // Play startup sound in parallel with showing the boot image

    with_writer(|writer| {
        // Clear screen with the background color.
        writer.clear(writer.background_color.to_rgb888()).unwrap();
        let img = include_bytes!("color_gradient.bmp");

        let image = Bmp::from_slice(img).unwrap();

        Image::new(&image, Point::new(0, 0)).draw(writer).unwrap();
    });
}

/// Boot finished: Clears the screen, plays a completion sound, and prints a welcome message.
/// After calling this function, all further output (like "Hello, world!" etc.) will be printed normally.
pub fn boot_finished() {
    with_writer(|writer| {
        // Clear the screen.
        writer.clear(writer.background_color.to_rgb888()).unwrap();
    });
}

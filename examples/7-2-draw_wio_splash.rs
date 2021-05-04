use embedded_graphics::{
    image::ImageRawLE, pixelcolor::Rgb565, prelude::*,
};
use embedded_graphics_simulator::*;
pub mod wio_splash;
use wio_splash::WioSplash;

// on host pc
// Comment out .cargo/config
fn main() {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("Wio Terminal Splash", &output_settings);

    let raw = ImageRawLE::new(include_bytes!("./assets/ferris.raw"), 86, 64);
    let splash = WioSplash::new(Rgb565::GREEN, raw);
    splash.draw(&mut display).unwrap();

    window.show_static(&display);
}

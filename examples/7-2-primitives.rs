use embedded_graphics::prelude::*;
use embedded_graphics::{pixelcolor::Rgb565,primitives::*,style::*};
use embedded_graphics_simulator::*;

// on host pc
// Comment out .cargo/config
fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(320, 240));
    let output_settings = OutputSettingsBuilder::new().build();
    let mut window = Window::new("draw a line", &output_settings);

    Line::new(Point::new(50, 20), Point::new(270, 220)).into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1)).draw(&mut display)?;
    Circle::new(Point::new(50, 200), 20).into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 5)).draw(&mut display)?;
    Triangle::new(Point::new(200, 20), Point::new(170, 45), Point::new(300, 150))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
        .draw(&mut display)?;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(10)
        .stroke_color(Rgb565::CYAN)
        .fill_color(Rgb565::YELLOW)
        .build();
    Rectangle::new(Point::new(100, 100), Point::new(220, 140))
        .into_styled(style)
        .draw(&mut display)?;

    window.show_static(&display);

    Ok(())
}

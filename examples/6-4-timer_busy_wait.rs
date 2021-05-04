//! 6-4 タイマ/割り込みのサンプルコードです。
//! 1秒間隔でLEDが点滅します
//!
//! ### 実行方法
//! ```sh
//! $ cargo hf2 --example 6-4-timer_busy_wait
//! ```

#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::hal::gpio::*; // GPIOの構造体やトレイトをインポートする
use wio::pac::{CorePeripherals, Peripherals};
use wio::prelude::*;
use wio::{entry, Pins, Sets};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();

    // LEDドライバオブジェクトを初期化する
    let mut sets: Sets = Pins::new(peripherals.PORT).split();
    let mut led = Led::new(sets.user_led, &mut sets.port);

    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        led.toggle();
        delay.delay_ms(1000u16);
    }
}
struct Led {
    pin: Pa15<Output<PushPull>>,
}
impl Led {
    fn new(pin: Pa15<Input<Floating>>, port: &mut Port) -> Led {
        Led {
            pin: pin.into_push_pull_output(port),
        }
    }
    fn toggle(&mut self) {
        self.pin.toggle();
    }
}

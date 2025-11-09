#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use nrf52840_hal::{
    self as hal,
    gpio::Level,
    pac,
    spim::{Spim, Pins as SpimPins, Frequency, MODE_0},
};
use embedded_hal::digital::v2::OutputPin;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

const NUM_LEDS: usize = 9;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    let mut led = port0.p0_06.into_push_pull_output(Level::Low).degrade();
    let _sk_pwr = port0.p0_25.into_push_pull_output(Level::High);

    // SPI pins for WS2812
    let sck = port0.p0_20.into_push_pull_output(Level::Low).degrade();
    let mosi = port0.p0_26.into_push_pull_output(Level::Low).degrade();

    let pins = SpimPins {
        sck: Some(sck),
        miso: None,
        mosi: Some(mosi),
    };

    let spi = Spim::new(p.SPIM0, pins, Frequency::M4, MODE_0, 0);
    let mut ws2812 = Ws2812::new(spi);

    let mut data = [RGB8::default(); NUM_LEDS];

    // Set all LEDs to white
    for led_color in data.iter_mut() {
        *led_color = RGB8 { r: 30, g: 20, b: 0 };
    }
    ws2812.write(data.iter().cloned()).ok();

    loop {
        led.set_high().ok();
        cortex_m::asm::delay(4_000_000);
        led.set_low().ok();
        cortex_m::asm::delay(4_000_000);
    }
}

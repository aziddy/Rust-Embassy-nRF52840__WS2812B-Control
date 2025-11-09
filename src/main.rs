#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // Configure LED pin (P0.06 for Adafruit nRF52840)
    let mut led = Output::new(p.P0_06, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high(); // Turn on
        Timer::after_millis(2000).await; // Brief blink
        led.set_low(); // Turn off
        Timer::after_millis(2000).await; // Brief blink
        led.set_high(); // Turn on
        Timer::after_millis(2000).await; // Brief blink
        led.set_low(); // Turn off
        Timer::after_secs(10).await; // Sleep 10 minutes (600 seconds)
    }
}

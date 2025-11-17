#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::{bind_interrupts, peripherals, spim};
use embassy_time::Timer;
use panic_probe as _;

use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

const NUM_LEDS: usize = 9;

bind_interrupts!(struct Irqs {
    SPIM3 => spim::InterruptHandler<peripherals::SPI3>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let sk_pwr = Output::new(p.P0_25, Level::High, OutputDrive::Standard);

    let mut config = spim::Config::default();
    config.frequency = spim::Frequency::M4;

    let spim = spim::Spim::new(p.SPI3, Irqs, p.P0_20, p.P0_28, p.P0_26, config);
    // Bit pattern that works for nRF52840 at 4MHz
    let custom_patterns = [0b01000100, 0b01000111, 0b01110100, 0b01110111];
    let mut ws2812 = Ws2812::new_with_custom_patterns(spim, custom_patterns);

    // Configure LED pin (P0.06 for Adafruit nRF52840)
    let mut led = Output::new(p.P0_06, Level::Low, OutputDrive::Standard);

    let mut data = [RGB8::default(); NUM_LEDS];
    defmt::info!("Start");

    data[0] = RGB8 { r: 2, g: 0, b: 0 };
    ws2812.write(data.iter().cloned()).ok();
    defmt::info!("Red - Not Bright");

    Timer::after_secs(1).await;

    data[1] = RGB8 { r: 0, g: 255, b: 0 };
    ws2812.write(data.iter().cloned()).ok();
    defmt::info!("Green");

    Timer::after_secs(1).await;

    data[2] = RGB8 { r: 0, g: 0, b: 255 };
    ws2812.write(data.iter().cloned()).ok();
    defmt::info!("Blue");

    Timer::after_secs(1).await;

    data[3] = RGB8 { r: 255, g: 0, b: 255 };
    ws2812.write(data.iter().cloned()).ok();
    defmt::info!("Purple");

    Timer::after_secs(1).await;

    data[4] = RGB8 { r: 0, g: 2, b: 2 };
    ws2812.write(data.iter().cloned()).ok();
    defmt::info!("Cyan - Not Bright");

    Timer::after_secs(1).await;

    data[5] = RGB8 { r: 255, g: 255, b: 0 };
    ws2812.write(data.iter().cloned()).ok();
    defmt::info!("Yellow");

    loop {
        defmt::info!("LOOP");

        // Red
        for i in 0..NUM_LEDS {
            data[i] = RGB8 { r: 255, g: 0, b: 0 };
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("Red");
        Timer::after_secs(1).await;

        // Green
        for i in 0..NUM_LEDS {
            data[i] = RGB8 { r: 0, g: 255, b: 0 };
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("Green");
        Timer::after_secs(1).await;

        // Blue
        for i in 0..NUM_LEDS {
            data[i] = RGB8 { r: 0, g: 0, b: 255 };
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("Blue");
        Timer::after_secs(1).await;

        // White
        for i in 0..NUM_LEDS {
            data[i] = RGB8 { r: 255, g: 255, b: 255 };
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("White");
        Timer::after_secs(1).await;

        // off
        for i in 0..NUM_LEDS {
            data[i] = RGB8::default();
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("Off");
        Timer::after_secs(1).await;
    }
}

#![no_std]
#![no_main]

use panic_probe as _;
use defmt_rtt as _;
use cortex_m_rt::entry;

defmt::timestamp!("{=u64}", {
    0
});
use nrf52840_hal::{
    self as hal,
    gpio::Level,
    pac,
    spim::{Spim, Pins as SpimPins, Frequency, MODE_0},
};
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_spi::Ws2812;

const NUM_LEDS: usize = 9;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);

    // Turn on mosfet controlling power to LEDS
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
    let custom_patterns = [0b01000100, 0b01000111, 0b01110100, 0b01110111];
    let mut ws2812 = Ws2812::new_with_custom_patterns(spi, custom_patterns);

    let mut data = [RGB8::default(); NUM_LEDS];

    defmt::info!("Start");
    data[0] = RGB8 { r: 2, g: 0, b: 0 };
    ws2812.write(data.iter().cloned()).ok();
    defmt::info!("Red - Not Bright");
    cortex_m::asm::delay(50_000_000);
    

    data[1] = RGB8 { r: 0, g: 255, b: 0 };
    ws2812.write(data.iter().cloned()).ok();
    cortex_m::asm::delay(50_000_000);
    defmt::info!("Green");

    data[2] = RGB8 { r: 0, g: 0, b: 255 };
    ws2812.write(data.iter().cloned()).ok();
    cortex_m::asm::delay(50_000_000);
    defmt::info!("Blue");

    data[3] = RGB8 { r: 255, g: 0, b: 255 };
    ws2812.write(data.iter().cloned()).ok();
    cortex_m::asm::delay(50_000_000);
    defmt::info!("Purple");

    data[4] = RGB8 { r: 0, g: 2, b: 2 };
    ws2812.write(data.iter().cloned()).ok();
    cortex_m::asm::delay(50_000_000);
    defmt::info!("Cyan - Not Bright");

    data[5] = RGB8 { r: 255, g: 255, b: 0 };
    ws2812.write(data.iter().cloned()).ok();
    defmt::info!("Yellow");
    cortex_m::asm::delay(50_000_000);
    

    loop {
        defmt::info!("LOOP");

        // Red
        for i in 0..NUM_LEDS {
            data[i] = RGB8 { r: 255, g: 0, b: 0 };
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("Red");
        cortex_m::asm::delay(24_000_000);

        // Green
        for i in 0..NUM_LEDS {
            data[i] = RGB8 { r: 0, g: 255, b: 0 };
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("Green");
        cortex_m::asm::delay(24_000_000);

        // Blue
        for i in 0..NUM_LEDS {
            data[i] = RGB8 { r: 0, g: 0, b: 255 };
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("Blue");
        cortex_m::asm::delay(24_000_000);

        // White
        for i in 0..NUM_LEDS {
            data[i] = RGB8 { r: 255, g: 255, b: 255 };
        }
        ws2812.write(data.iter().cloned()).ok();
        defmt::info!("Blue");
        cortex_m::asm::delay(24_000_000);

        // off
        for i in 0..NUM_LEDS {
            data[i] = RGB8::default();
        }
        ws2812.write(data.iter().cloned()).ok();
        cortex_m::asm::delay(24_000_000);
    }
}

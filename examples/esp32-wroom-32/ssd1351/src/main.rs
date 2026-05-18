//! SSD1351 OLED display example for ESP32-WROOM via SPI.
//!
//! Note: SSD1351 is a 128x128, 65K color (RGB565) display.
//! Features 256-step contrast per color channel and 16-step master current.
//! SSD1351 serial interface is write-only (SPI).
//!
//! Wiring:
//! - GPIO23 (MOSI) → OLED SDA/SDIN
//! - GPIO18 (SCLK) → OLED SCL/SCLK
//! - GPIO5  (CS)   → OLED CS#
//! - GPIO16 (D/C)  → OLED D/C#
//! - GPIO17 (RES)  → OLED RES# (active low reset)
//! - 10-18V → OLED VCC (external supply)
//! - GND → OLED GND

#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::Text,
};
use esp_hal::{
    gpio::{Output, OutputConfig},
    main,
    spi::master::{Config as SpiConfig, Spi},
    time::Rate,
};
use oled_ssd_driver::{SpiInterface, Ssd1351};

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default().with_frequency(Rate::from_mhz(8)),
    )
    .unwrap()
    .with_mosi(peripherals.GPIO23)
    .with_sck(peripherals.GPIO18);

    let cs = Output::new(
        peripherals.GPIO5,
        esp_hal::gpio::Level::High,
        OutputConfig::default(),
    );
    let dc = Output::new(
        peripherals.GPIO16,
        esp_hal::gpio::Level::Low,
        OutputConfig::default(),
    );
    let mut res = Output::new(
        peripherals.GPIO17,
        esp_hal::gpio::Level::High,
        OutputConfig::default(),
    );

    // Hardware reset (SSD1351 §8.9 — RES# low for at least 2us)
    res.set_low();
    for _ in 0..1000 {
        core::hint::spin_loop();
    }
    res.set_high();
    // Wait at least 300ms after VCC stable before sending commands
    for _ in 0..100_000 {
        core::hint::spin_loop();
    }

    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let interface = SpiInterface::new(spi_device, dc);
    let mut display = Ssd1351::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text (white)
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    Text::new("Hello SSD1351!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line (green)
    Line::new(Point::new(0, 20), Point::new(127, 127))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (blue outline)
    Circle::new(Point::new(40, 40), 60)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 2))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

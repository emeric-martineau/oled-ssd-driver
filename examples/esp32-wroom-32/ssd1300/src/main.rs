//! SSD1300 OLED display example for ESP32-WROOM via SPI.
//!
//! Note: SSD1300 is a 104x48, monochrome display with internal DC-DC booster.
//! Features 256-step contrast, 4 area colour selections, and horizontal scrolling.
//! SSD1300 serial interface is write-only (SPI).
//!
//! Wiring:
//! - GPIO23 (MOSI) → OLED SDA/SDIN (D1)
//! - GPIO18 (SCLK) → OLED SCL/SCLK (D0)
//! - GPIO5  (CS)   → OLED CS#
//! - GPIO16 (D/C)  → OLED D/C#
//! - GPIO17 (RES)  → OLED RES# (active low reset)
//! - 7-16V → OLED VCC (external supply, or use internal DC-DC booster)
//! - GND → OLED GND

#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
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
use oled_ssd_driver::{SpiInterface, Ssd1300};

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

    // Hardware reset
    res.set_low();
    // brief delay (at least 3us per datasheet)
    for _ in 0..1000 {
        core::hint::spin_loop();
    }
    res.set_high();
    for _ in 0..1000 {
        core::hint::spin_loop();
    }

    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let interface = SpiInterface::new(spi_device, dc);
    let mut display = Ssd1300::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello SSD1300!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line
    Line::new(Point::new(0, 16), Point::new(103, 47))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (outline)
    Circle::new(Point::new(60, 10), 30)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

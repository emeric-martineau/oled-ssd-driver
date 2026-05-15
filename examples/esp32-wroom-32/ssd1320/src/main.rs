//! SSD1320 OLED display example for ESP32-WROOM via I2C.
//!
//! Note: SSD1320 is a 160x160, 16 grayscale (4-bit per pixel) display.
//! Requires external VCC (8-18V).
//!
//! Wiring:
//! - GPIO21 (SDA) → OLED SDA
//! - GPIO22 (SCL) → OLED SCL
//! - 8-18V → OLED VCC (external supply)
//! - GND → OLED GND

#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Gray4,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::Text,
};
use esp_hal::{i2c::master::I2c, main, time::Rate};
use oled_ssd_driver::{I2cInterface, Ssd1320, DEFAULT_I2C_ADDRESS};

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)))
        .unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
    let mut display = Ssd1320::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text (full white on grayscale)
    let text_style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
    Text::new("Hello SSD1320!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line (medium gray)
    Line::new(Point::new(0, 20), Point::new(159, 159))
        .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x0A), 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (light gray outline)
    Circle::new(Point::new(80, 60), 60)
        .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x0C), 2))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

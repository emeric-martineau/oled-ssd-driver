//! SSD0303 OLED display example for ESP32-WROOM via I2C.
//!
//! Note: SSD0303 is a 132x64, monochrome display with internal DC-DC voltage converter.
//! Features 256-step contrast, 4 area colour selections, and horizontal scrolling.
//! I2C slave address: 0x3C (SA0=0) or 0x3D (SA0=1).
//!
//! Wiring:
//! - GPIO21 (SDA) → OLED SDA
//! - GPIO22 (SCL) → OLED SCL
//! - 3.3V → OLED VDD
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
use esp_hal::{i2c::master::I2c, main, time::Rate};
use oled_ssd_driver::{I2cInterface, Ssd0303, DEFAULT_I2C_ADDRESS};

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
    let mut display = Ssd0303::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello SSD0303!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line
    Line::new(Point::new(0, 16), Point::new(131, 63))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (outline)
    Circle::new(Point::new(70, 15), 40)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

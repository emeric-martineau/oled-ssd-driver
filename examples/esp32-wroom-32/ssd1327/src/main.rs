//! SSD1327 OLED display example for ESP32-WROOM via I2C.
//!
//! Note: SSD1327 is a 128x128, 16 grayscale (4-bit per pixel) display.
//! Features internal VDD regulator and 256-step contrast.
//!
//! Wiring:
//! - GPIO21 (SDA) → OLED SDA
//! - GPIO22 (SCL) → OLED SCL
//! - 8-18V → OLED VCC (external supply)
//! - GND → OLED GND

#![no_std]
#![no_main]

use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Gray4,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::Text,
};
use esp_hal::{i2c::master::I2c, main, time::Rate};
use oled_ssd_driver::{I2cInterface, Ssd1327, DEFAULT_I2C_ADDRESS};

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
    let mut display = Ssd1327::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text (full white on grayscale)
    let text_style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
    Text::new("Hello SSD1327!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line (medium gray)
    Line::new(Point::new(0, 20), Point::new(127, 97))
        .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x0A), 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (light gray outline)
    Circle::new(Point::new(60, 50), 30)
        .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x0C), 2))
        .draw(&mut display)
        .unwrap();

    // Draw a 16x16 bitmap image (Rust gear icon, 4bpp, 8 bytes per row)
    #[rustfmt::skip]
    const RUST_ICON: &[u8] = &[
        // Line 1
        0b0000_0000, 0b0000_0000, 0b0000_1111, 0b1111_0000, 0b0000_1111, 0b1111_0000, 0b0000_0000, 0b0000_0000,
        // Line 2
        0b0000_0000, 0b0000_0000, 0b0000_1111, 0b1111_0000, 0b0000_1111, 0b1111_0000, 0b0000_0000, 0b0000_0000,
        // Line 3
        0b0000_0000, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0000_0000,
        // Line 4
        0b0000_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_0000,
        // Line 5
        0b0000_1111, 0b1111_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_1111, 0b1111_0000,
        // Line 6
        0b1111_1111, 0b1111_0000, 0b0000_1111, 0b1111_1111, 0b1111_1111, 0b1111_0000, 0b0000_1111, 0b1111_1111,
        // Line 7
        0b1111_0000, 0b0000_0000, 0b0000_1111, 0b0000_0000, 0b0000_0000, 0b1111_0000, 0b0000_0000, 0b1111_1111,
        // Line 8
        0b1111_0000, 0b0000_0000, 0b0000_1111, 0b1111_1111, 0b1111_1111, 0b1111_0000, 0b0000_0000, 0b1111_1111,
        // Line 9
        0b1111_0000, 0b0000_0000, 0b0000_1111, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b1111_1111,
        // Line 10
        0b1111_0000, 0b0000_0000, 0b0000_1111, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b1111_1111,
        // Line 11
        0b1111_1111, 0b1111_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b1111_1111, 0b1111_1111,
        // Line 12
        0b0000_1111, 0b1111_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0000_1111, 0b1111_0000,
        // Line 13
        0b0000_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_0000,
        // Line 14
        0b0000_0000, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b1111_1111, 0b0000_0000,
        // Line 15
        0b0000_0000, 0b0000_0000, 0b0000_1111, 0b1111_0000, 0b0000_1111, 0b1111_0000, 0b0000_0000, 0b0000_0000,
        // Line 16
        0b0000_0000, 0b0000_0000, 0b0000_1111, 0b1111_0000, 0b0000_1111, 0b1111_0000, 0b0000_0000, 0b0000_0000,
    ];
    let raw_image = ImageRaw::<Gray4>::new(RUST_ICON, 16);
    Image::new(&raw_image, Point::new(4, 80))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

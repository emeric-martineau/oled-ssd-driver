//! SSD1306 OLED 128x32 display example for ESP32-WROOM via I2C.
//!
//! Wiring:
//! - GPIO21 (SDA) → OLED SDA
//! - GPIO22 (SCL) → OLED SCL
//! - 3.3V → OLED VCC
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
use oled_ssd_driver::{I2cInterface, Ssd1306, DEFAULT_I2C_ADDRESS, SIZE_128X32};

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
    let mut display = Ssd1306::new_with_size(interface, SIZE_128X32);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text (fits in 32px height)
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello 128x32!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line across the display
    Line::new(Point::new(0, 16), Point::new(127, 31))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a small circle
    Circle::new(Point::new(100, 8), 20)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

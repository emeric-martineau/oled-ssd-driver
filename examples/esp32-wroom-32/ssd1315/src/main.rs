//! SSD1315 OLED display example for ESP32-WROOM via I2C.
//!
//! Wiring:
//! - GPIO21 (SDA) → OLED SDA
//! - GPIO22 (SCL) → OLED SCL
//! - 3.3V → OLED VCC
//! - GND → OLED GND

#![no_std]
#![no_main]

use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::Text,
};
use esp_hal::{i2c::master::I2c, main, time::Rate};
use oled_ssd_driver::{ChargePumpVoltage1315, I2cInterface, Ssd1315, DEFAULT_I2C_ADDRESS};

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
    let mut display = Ssd1315::new(interface);

    display.init().unwrap();
    display.clear_buffer();

    // SSD1315-specific: set charge pump to 8.5V for brighter output
    display
        .set_charge_pump_voltage(ChargePumpVoltage1315::V8_5)
        .unwrap();

    // Draw text
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello SSD1315!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line
    Line::new(Point::new(0, 16), Point::new(127, 40))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (outline)
    Circle::new(Point::new(90, 20), 30)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a 16x16 bitmap image (Rust gear icon, 1bpp, 2 bytes per row)
    #[rustfmt::skip]
    const RUST_ICON: &[u8] = &[
        0b0000_0110, 0b0110_0000,
        0b0000_0110, 0b0110_0000,
        0b0011_1111, 0b1111_1100,
        0b0111_1111, 0b1111_1110,
        0b0110_0000, 0b0000_0110,
        0b1110_0111, 0b1110_0111,
        0b1100_0100, 0b0010_0011,
        0b1100_0111, 0b1110_0011,
        0b1100_0100, 0b0000_0011,
        0b1100_0100, 0b0000_0011,
        0b1110_0000, 0b0000_0111,
        0b0110_0000, 0b0000_0110,
        0b0111_1111, 0b1111_1110,
        0b0011_1111, 0b1111_1100,
        0b0000_0110, 0b0110_0000,
        0b0000_0110, 0b0110_0000,
    ];
    let raw_image = ImageRaw::<BinaryColor>::new(RUST_ICON, 16);
    Image::new(&raw_image, Point::new(4, 44))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

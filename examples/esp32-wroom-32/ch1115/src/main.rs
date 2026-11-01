//! CH1115 OLED display example for ESP32-WROOM via I2C.
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
use oled_ssd_driver::{Ch1115, I2cInterface, PumpVoltageCh1115, DEFAULT_I2C_ADDRESS};

#[panic_handler]
fn panic(_panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

esp_bootloader_esp_idf::esp_app_desc!();

pub const DISPLAY_OFFSET_X: i32 = 40;
pub const DISPLAY_OFFSET_Y: i32 = 14;

#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let i2c = I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default().with_frequency(Rate::from_khz(400)))
        .unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
    let mut display = Ch1115::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // CH1115-specific: set pump voltage to 9.0V for brighter output
    display
        .set_pump_voltage(PumpVoltageCh1115::V9_0)
        .unwrap();

    // Draw text
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello CH1115!", Point::new(DISPLAY_OFFSET_X, DISPLAY_OFFSET_Y), text_style)
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

    display.flush().unwrap();

    loop {}
}

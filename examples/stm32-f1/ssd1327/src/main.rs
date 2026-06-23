//! SSD1327 OLED display example for STM32F103 "Blue Pill" via I2C.
//!
//! Note: SSD1327 is a 128x128, 16 grayscale (4-bit per pixel) display.
//! Features internal VDD regulator and 256-step contrast.
//!
//! Wiring:
//! - PB6 (SCL) → OLED SCL
//! - PB7 (SDA) → OLED SDA
//! - 8-18V → OLED VCC (external supply)
//! - GND → OLED GND

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Gray4,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::Text,
};
use oled_ssd_driver::{I2cInterface, Ssd1327, DEFAULT_I2C_ADDRESS};
use panic_halt as _;
use stm32f1xx_hal::{
    i2c::Mode,
    pac,
    prelude::*,
};
use stm32f1xx_hal::rcc::Config;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp
        .RCC
        .freeze(Config::hsi().adcclk(2.MHz()), &mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc);

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = dp
        .I2C1
        .blocking_i2c(
            (scl, sda),
            Mode::Fast {
                frequency: 400.kHz(),
                duty_cycle: stm32f1xx_hal::i2c::DutyCycle::Ratio2to1,
            },
            &mut rcc,
            1000,
            10,
            1000,
            1000,
        );

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
    Line::new(Point::new(0, 20), Point::new(127, 127))
        .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x0A), 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (light gray outline)
    Circle::new(Point::new(30, 30), 60)
        .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x0C), 2))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

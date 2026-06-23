//! SSD1325 OLED display example for STM32F103 "Blue Pill" via I2C.
//!
//! Note: SSD1325 is a 128x80, 16 grayscale (4-bit per pixel) display.
//! Requires external VCC (8-16V) and external IREF via resistor.
//!
//! Wiring:
//! - PB6 (SCL) → OLED SCL
//! - PB7 (SDA) → OLED SDA
//! - 8-16V → OLED VCC (external supply)
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
use oled_ssd_driver::{I2cInterface, Ssd1325, DEFAULT_I2C_ADDRESS};
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
    let mut display = Ssd1325::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text (full white on grayscale)
    let text_style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
    Text::new("Hello SSD1325!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line (medium gray)
    Line::new(Point::new(0, 20), Point::new(127, 79))
        .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x0A), 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (light gray outline)
    Circle::new(Point::new(50, 20), 50)
        .into_styled(PrimitiveStyle::with_stroke(Gray4::new(0x0C), 2))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

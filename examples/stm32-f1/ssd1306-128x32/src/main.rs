//! SSD1306 OLED 128x32 display example for STM32F103 "Blue Pill" via I2C.
//!
//! Wiring:
//! - PB6 (SCL) → OLED SCL
//! - PB7 (SDA) → OLED SDA
//! - 3.3V → OLED VCC
//! - GND → OLED GND

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::Text,
};
use oled_ssd_driver::{I2cInterface, Ssd1306, DEFAULT_I2C_ADDRESS, SIZE_128X32};
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

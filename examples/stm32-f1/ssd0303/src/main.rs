//! SSD0303 OLED display example for STM32F103 "Blue Pill" via I2C.
//!
//! Note: SSD0303 is a 132x64, monochrome OLED/PLED display with internal DC-DC converter.
//! Supports 4 area colour selections and horizontal scrolling.
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
use oled_ssd_driver::{I2cInterface, Ssd0303, DEFAULT_I2C_ADDRESS};
use panic_halt as _;
use stm32f1xx_hal::{
    i2c::{BlockingI2c, Mode},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain();
    let mut gpiob = dp.GPIOB.split();

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.Hz(),
            duty_cycle: stm32f1xx_hal::i2c::DutyCycle::Ratio2to1,
        },
        clocks,
        1000,
        10,
        1000,
        1000,
    );

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
    Line::new(Point::new(0, 16), Point::new(131, 50))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (outline)
    Circle::new(Point::new(80, 20), 30)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

//! SSD1300 OLED display example for STM32F103 "Blue Pill" via SPI.
//!
//! Note: SSD1300 is a 104x48, monochrome OLED/PLED display with internal DC-DC booster.
//! Supports 4 area colour selections and horizontal scrolling.
//! SSD1300 serial interface is write-only (SPI).
//!
//! Wiring:
//! - PA5 (SCK)  → OLED SCLK
//! - PA7 (MOSI) → OLED SDIN
//! - PA4 (CS)   → OLED CS#
//! - PB0 (D/C)  → OLED D/C#
//! - PB1 (RES)  → OLED RES# (active low reset)
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
use embedded_hal_bus::spi::ExclusiveDevice;
use oled_ssd_driver::{SpiInterface, Ssd1300};
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, spi, spi::Spi};
use stm32f1xx_hal::rcc::Config;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp
        .RCC
        .freeze(Config::hsi().adcclk(2.MHz()), &mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpiob = dp.GPIOB.split(&mut rcc);

    // SPI1 pins
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6;
    let mosi = gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl);

    // CS and D/C pins
    let cs = gpioa.pa4.into_push_pull_output(&mut gpioa.crl);
    let dc = gpiob.pb0.into_push_pull_output(&mut gpiob.crl);
    let mut res = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);

    // Hardware reset
    res.set_low();
    cortex_m::asm::delay(72_000); // ~1ms at 72MHz
    res.set_high();
    cortex_m::asm::delay(72_000);

    let spi = Spi::new(
        dp.SPI1,
        (Some(sck), Some(miso), Some(mosi)),
        spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        },
        8.MHz(),
        &mut rcc,
    );

    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();
    let interface = SpiInterface::new(spi_device, dc);
    let mut display = Ssd1300::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello SSD1300!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line
    Line::new(Point::new(0, 16), Point::new(103, 47))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (outline)
    Circle::new(Point::new(60, 15), 25)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

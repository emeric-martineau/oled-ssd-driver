//! SSD1332 OLED display example for STM32F103 "Blue Pill" via SPI.
//!
//! Note: SSD1332 is a 96x64, 65K color (RGB565) display with internal DC-DC booster.
//! Features 256-step contrast per color channel and 16-step master current.
//! SSD1332 serial interface is write-only (SPI).
//!
//! Wiring:
//! - PA5 (SCK)  → OLED SCLK
//! - PA7 (MOSI) → OLED SDIN
//! - PA4 (CS)   → OLED CS#
//! - PB0 (D/C)  → OLED D/C#
//! - PB1 (RES)  → OLED RES# (active low reset)
//! - 8-18V → OLED VCC (external supply, or use internal booster)
//! - GND → OLED GND

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::Text,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use oled_ssd_driver::{SpiInterface, Ssd1332};
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*, spi, spi::Spi};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    // SPI1 pins
    let sck = gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl);
    let miso = gpioa.pa6; // not used but required by HAL
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

    let spi = Spi::spi1(
        dp.SPI1,
        (sck, miso, mosi),
        &mut gpioa.crl,
        spi::Mode {
            polarity: spi::Polarity::IdleLow,
            phase: spi::Phase::CaptureOnFirstTransition,
        },
        8.MHz(),
        &clocks,
    );

    let spi_device = ExclusiveDevice::new_no_delay(spi, cs);
    let interface = SpiInterface::new(spi_device, dc);
    let mut display = Ssd1332::new(interface);

    display.init(&Default::default()).unwrap();
    display.clear_buffer();

    // Draw text (white)
    let text_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    Text::new("Hello SSD1332!", Point::new(0, 10), text_style)
        .draw(&mut display)
        .unwrap();

    // Draw a diagonal line (green)
    Line::new(Point::new(0, 20), Point::new(95, 63))
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
        .draw(&mut display)
        .unwrap();

    // Draw a circle (blue outline)
    Circle::new(Point::new(40, 20), 40)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 2))
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {}
}

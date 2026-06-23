//! SSD1312 OLED display example for STM32F103 "Blue Pill" via I2C.
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
    image::{Image, ImageRaw},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle},
    text::Text,
};
use oled_ssd_driver::{ChargePumpVoltage1312, I2cInterface, InitConfig, Ssd1312, DEFAULT_I2C_ADDRESS};
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
    let mut display = Ssd1312::new(interface);

    let config = InitConfig {
        segment_remap: false,
        com_scan_reversed: false,
        ..Default::default()
    };
    display.init(&config).unwrap();
    display.clear_buffer();

    // SSD1312-specific: set charge pump to 8.5V for brighter output
    display
        .set_charge_pump_voltage(ChargePumpVoltage1312::V8_5)
        .unwrap();

    // Draw text
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hello SSD1312!", Point::new(0, 10), text_style)
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

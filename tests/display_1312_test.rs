use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use oled_ssd_driver::{
    ChargePumpVoltage1312, IrefSetting1312, ScrollDirection, ScrollInterval1312, Ssd1312,
    VcomhLevel1312,
};

// Default display width (128 pixels)
pub const WIDTH: u8 = 128;
// Default display height (64 pixels)
pub const HEIGHT: u8 = 64;

struct MockInterface {
    commands: Vec<Vec<u8>>,
    data: Vec<Vec<u8>>,
}

impl MockInterface {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
            data: Vec::new(),
        }
    }
}

impl oled_ssd_driver::interface::DisplayInterface for MockInterface {
    fn send_command(&mut self, cmd: &[u8]) -> Result<(), oled_ssd_driver::DisplayError> {
        self.commands.push(cmd.to_vec());
        Ok(())
    }

    fn send_data(&mut self, data: &[u8]) -> Result<(), oled_ssd_driver::DisplayError> {
        self.data.push(data.to_vec());
        Ok(())
    }
}

#[test]
fn test_1312_dimensions() {
    let display = Ssd1312::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(128, 64));
}

#[test]
fn test_1312_set_pixel() {
    let mut display = Ssd1312::new(MockInterface::new());
    display.set_pixel(0, 0, true);
    assert_eq!(display.buffer()[0], 0x01);

    display.set_pixel(127, 63, true);
    let idx = 7 * WIDTH as usize + 127;
    assert_eq!(display.buffer()[idx], 0x80);
}

#[test]
fn test_1312_init_sends_iref_and_charge_pump() {
    let mut display = Ssd1312::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    // Should contain IREF setting (ADh, 10h)
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x10]));
    // Should contain charge pump 7.5V (8Dh, 14h)
    assert!(iface.commands.iter().any(|c| c == &[0x8D, 0x14]));
    // Should end with display ON
    assert!(iface.commands.iter().any(|c| c == &[0xAF]));
    assert!(iface.data.is_empty());
}

#[test]
fn test_1312_flush_sends_data() {
    let mut display = Ssd1312::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_pixel(0, 0, true);
    display.flush().unwrap();

    let iface = display.release();
    assert!(!iface.data.is_empty());
    let total_data: usize = iface.data.iter().map(|d| d.len()).sum();
    assert_eq!(total_data, WIDTH as usize * (HEIGHT as usize / 8));
}

#[test]
fn test_1312_set_charge_pump_voltage_v9() {
    let mut display = Ssd1312::new(MockInterface::new());
    display
        .set_charge_pump_voltage(ChargePumpVoltage1312::V9_0)
        .unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x8D, 0x95]));
}

#[test]
fn test_1312_set_charge_pump_voltage_v10() {
    let mut display = Ssd1312::new(MockInterface::new());
    display
        .set_charge_pump_voltage(ChargePumpVoltage1312::V10_0)
        .unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x8D, 0x15]));
}

#[test]
fn test_1312_set_charge_pump_voltage_v8_5() {
    let mut display = Ssd1312::new(MockInterface::new());
    display
        .set_charge_pump_voltage(ChargePumpVoltage1312::V8_5)
        .unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x8D, 0x94]));
}

#[test]
fn test_1312_set_iref_internal() {
    let mut display = Ssd1312::new(MockInterface::new());
    display.set_iref(IrefSetting1312::Internal).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x10]));
}

#[test]
fn test_1312_set_iref_external() {
    let mut display = Ssd1312::new(MockInterface::new());
    display.set_iref(IrefSetting1312::External).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x00]));
}

#[test]
fn test_1312_set_vcomh_deselect() {
    let mut display = Ssd1312::new(MockInterface::new());
    display.set_vcomh_deselect(VcomhLevel1312::V071).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xDB, 0x10]));
}

#[test]
fn test_1312_horizontal_scroll_with_columns() {
    let mut display = Ssd1312::new(MockInterface::new());
    display
        .start_horizontal_scroll(
            ScrollDirection::Right,
            0,
            7,
            ScrollInterval1312::Frames6,
            10,
            100,
        )
        .unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x2E]));
    assert!(iface
        .commands
        .iter()
        .any(|c| c == &[0x26, 0x00, 0x00, 0x00, 0x07, 0x0A, 0x64]));
    assert!(iface.commands.iter().any(|c| c == &[0x2F]));
}

#[test]
fn test_1312_content_scroll() {
    let mut display = Ssd1312::new(MockInterface::new());
    display
        .start_content_scroll(
            ScrollDirection::Left,
            1,
            5,
            ScrollInterval1312::Frames32,
            0,
            127,
        )
        .unwrap();

    let iface = display.release();
    assert!(iface
        .commands
        .iter()
        .any(|c| c == &[0x2D, 0x00, 0x01, 0x01, 0x05, 0x00, 0x7F]));
    assert!(iface.commands.iter().any(|c| c == &[0x2F]));
}

#[test]
fn test_1312_horizontal_scroll_invalid_pages() {
    let mut display = Ssd1312::new(MockInterface::new());
    let result = display.start_horizontal_scroll(
        ScrollDirection::Right,
        5,
        3,
        ScrollInterval1312::Frames6,
        0,
        127,
    );
    assert_eq!(result, Err(oled_ssd_driver::DisplayError::InvalidArgument));
}

#[test]
fn test_1312_horizontal_scroll_invalid_columns() {
    let mut display = Ssd1312::new(MockInterface::new());
    let result = display.start_horizontal_scroll(
        ScrollDirection::Right,
        0,
        7,
        ScrollInterval1312::Frames6,
        100,
        50,
    );
    assert_eq!(result, Err(oled_ssd_driver::DisplayError::InvalidArgument));
}

#[test]
fn test_1312_draw_target_pixel() {
    let mut display = Ssd1312::new(MockInterface::new());
    let pixels = [
        Pixel(Point::new(0, 0), BinaryColor::On),
        Pixel(Point::new(1, 8), BinaryColor::On),
        Pixel(Point::new(-1, 0), BinaryColor::On),
    ];
    display.draw_iter(pixels).unwrap();

    assert_eq!(display.buffer()[0], 0x01);
    let idx = WIDTH as usize + 1;
    assert_eq!(display.buffer()[idx], 0x01);
}

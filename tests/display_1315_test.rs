use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use oled_ssd_driver::{
    ChargePumpVoltage1315, FadeMode1315, IrefSetting1315, ScrollDirection, ScrollInterval1315,
    Ssd1315, VcomhLevel1315, HEIGHT, WIDTH,
};

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
fn test_1315_dimensions() {
    let display = Ssd1315::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(128, 64));
}

#[test]
fn test_1315_set_pixel() {
    let mut display = Ssd1315::new(MockInterface::new());
    display.set_pixel(0, 0, true);
    assert_eq!(display.buffer()[0], 0x01);

    display.set_pixel(127, 63, true);
    let idx = 7 * WIDTH as usize + 127;
    assert_eq!(display.buffer()[idx], 0x80);
}

#[test]
fn test_1315_init_sends_iref_and_charge_pump() {
    let mut display = Ssd1315::new(MockInterface::new());
    display.init().unwrap();

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
fn test_1315_flush_sends_data() {
    let mut display = Ssd1315::new(MockInterface::new());
    display.init().unwrap();
    display.set_pixel(0, 0, true);
    display.flush().unwrap();

    let iface = display.release();
    assert!(!iface.data.is_empty());
    let total_data: usize = iface.data.iter().map(|d| d.len()).sum();
    assert_eq!(total_data, WIDTH as usize * (HEIGHT as usize / 8));
}

#[test]
fn test_1315_set_charge_pump_voltage() {
    let mut display = Ssd1315::new(MockInterface::new());
    display
        .set_charge_pump_voltage(ChargePumpVoltage1315::V9_0)
        .unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x8D, 0x95]));
}

#[test]
fn test_1315_set_charge_pump_voltage_v8_5() {
    let mut display = Ssd1315::new(MockInterface::new());
    display
        .set_charge_pump_voltage(ChargePumpVoltage1315::V8_5)
        .unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x8D, 0x94]));
}

#[test]
fn test_1315_set_iref() {
    let mut display = Ssd1315::new(MockInterface::new());
    display.set_iref(IrefSetting1315::High).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x30]));
}

#[test]
fn test_1315_set_vcomh_deselect() {
    let mut display = Ssd1315::new(MockInterface::new());
    display.set_vcomh_deselect(VcomhLevel1315::V071).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xDB, 0x10]));
}

#[test]
fn test_1315_set_fade_blink() {
    let mut display = Ssd1315::new(MockInterface::new());
    display.set_fade_blink(FadeMode1315::Blink, 5).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x23, 0x35]));
}

#[test]
fn test_1315_set_fade_blink_invalid_interval() {
    let mut display = Ssd1315::new(MockInterface::new());
    let result = display.set_fade_blink(FadeMode1315::FadeOut, 0x10);
    assert_eq!(result, Err(oled_ssd_driver::DisplayError::InvalidArgument));
}

#[test]
fn test_1315_set_zoom() {
    let mut display = Ssd1315::new(MockInterface::new());
    display.set_zoom(true).unwrap();
    display.set_zoom(false).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xD6, 0x01]));
    assert!(iface.commands.iter().any(|c| c == &[0xD6, 0x00]));
}

#[test]
fn test_1315_horizontal_scroll_with_columns() {
    let mut display = Ssd1315::new(MockInterface::new());
    display
        .start_horizontal_scroll(
            ScrollDirection::Right,
            0,
            7,
            ScrollInterval1315::Frames6,
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
fn test_1315_content_scroll() {
    let mut display = Ssd1315::new(MockInterface::new());
    display
        .start_content_scroll(
            ScrollDirection::Left,
            1,
            5,
            ScrollInterval1315::Frames32,
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
fn test_1315_horizontal_scroll_invalid_pages() {
    let mut display = Ssd1315::new(MockInterface::new());
    let result = display.start_horizontal_scroll(
        ScrollDirection::Right,
        5,
        3,
        ScrollInterval1315::Frames6,
        0,
        127,
    );
    assert_eq!(result, Err(oled_ssd_driver::DisplayError::InvalidArgument));
}

#[test]
fn test_1315_horizontal_scroll_invalid_columns() {
    let mut display = Ssd1315::new(MockInterface::new());
    let result = display.start_horizontal_scroll(
        ScrollDirection::Right,
        0,
        7,
        ScrollInterval1315::Frames6,
        100,
        50,
    );
    assert_eq!(result, Err(oled_ssd_driver::DisplayError::InvalidArgument));
}

#[test]
fn test_1315_draw_target_pixel() {
    let mut display = Ssd1315::new(MockInterface::new());
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

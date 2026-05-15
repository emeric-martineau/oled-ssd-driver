use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use oled_ssd_driver::Ssd1309;

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
fn test_1309_dimensions() {
    let display = Ssd1309::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(128, 64));
}

#[test]
fn test_1309_pages() {
    let display = Ssd1309::new(MockInterface::new());
    // 64 pixels high → 64/8 = 8 pages
    assert_eq!(display.size_config().width, 128);
    assert_eq!(display.size_config().height, 64);
}

#[test]
fn test_1309_set_pixel_in_bounds() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.set_pixel(0, 0, true);
    assert_eq!(display.buffer()[0], 0x01);

    display.set_pixel(0, 7, true);
    assert_eq!(display.buffer()[0], 0x81);

    // Last valid pixel: x=127, y=63 (page 7, bit 7)
    display.set_pixel(127, 63, true);
    let idx = 7 * 128 + 127;
    assert_eq!(display.buffer()[idx], 0x80);
}

#[test]
fn test_1309_set_pixel_out_of_bounds() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.set_pixel(128, 0, true);
    display.set_pixel(0, 64, true);
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_1309_clear_buffer() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.set_pixel(10, 10, true);
    display.clear_buffer();
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_1309_draw_target_pixel() {
    let mut display = Ssd1309::new(MockInterface::new());
    let pixels = [
        Pixel(Point::new(0, 0), BinaryColor::On),
        Pixel(Point::new(1, 8), BinaryColor::On),
        Pixel(Point::new(-1, 0), BinaryColor::On), // out of bounds, ignored
        Pixel(Point::new(0, 64), BinaryColor::On), // out of bounds, ignored
    ];
    display.draw_iter(pixels).unwrap();

    assert_eq!(display.buffer()[0], 0x01);
    let idx = 128 + 1;
    assert_eq!(display.buffer()[idx], 0x01);
}

#[test]
fn test_1309_init_sends_commands() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    assert!(!iface.commands.is_empty());
    assert!(iface.data.is_empty());
}

#[test]
fn test_1309_init_no_charge_pump() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    // SSD1309 has no charge pump — command 0x8D should NOT be sent
    assert!(!iface.commands.iter().any(|c| c.first() == Some(&0x8D)));
}

#[test]
fn test_1309_init_multiplex_ratio() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    // Multiplex ratio should be height-1 = 63 (0x3F)
    assert!(iface.commands.iter().any(|c| c == &[0xA8, 63]));
}

#[test]
fn test_1309_flush_sends_data() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_pixel(0, 0, true);
    display.flush().unwrap();

    let iface = display.release();
    assert!(!iface.data.is_empty());
    // 8 pages × 128 columns = 1024 bytes
    let total_data: usize = iface.data.iter().map(|d| d.len()).sum();
    assert_eq!(total_data, 128 * 8);
}

#[test]
fn test_1309_set_contrast() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.set_contrast(0xFF).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x81, 0xFF]));
}

#[test]
fn test_1309_display_on_off() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.display_on(false).unwrap();
    display.display_on(true).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAE]));
    assert!(iface.commands.iter().any(|c| c == &[0xAF]));
}

#[test]
fn test_1309_set_inverse() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.set_inverse(true).unwrap();
    display.set_inverse(false).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xA7]));
    assert!(iface.commands.iter().any(|c| c == &[0xA6]));
}

#[test]
fn test_1309_horizontal_scroll() {
    use oled_ssd_driver::{ScrollDirection, ScrollInterval};

    let mut display = Ssd1309::new(MockInterface::new());
    display
        .start_horizontal_scroll(ScrollDirection::Right, 0, 7, ScrollInterval::Frames5)
        .unwrap();

    let iface = display.release();
    // Deactivate scroll (0x2E) then scroll setup (0x26...) then activate (0x2F)
    assert!(iface.commands.iter().any(|c| c == &[0x2E]));
    assert!(iface.commands.iter().any(|c| c == &[0x2F]));
}

#[test]
fn test_1309_horizontal_scroll_invalid_pages() {
    use oled_ssd_driver::{ScrollDirection, ScrollInterval};

    let mut display = Ssd1309::new(MockInterface::new());
    // Page 8 is invalid for 128x64 (only pages 0-7)
    let result =
        display.start_horizontal_scroll(ScrollDirection::Right, 0, 8, ScrollInterval::Frames5);
    assert!(result.is_err());
}

#[test]
fn test_1309_stop_scroll() {
    let mut display = Ssd1309::new(MockInterface::new());
    display.stop_scroll().unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x2E]));
}

#[test]
fn test_1309_set_vcomh_deselect() {
    use oled_ssd_driver::VcomhLevel1309;

    let mut display = Ssd1309::new(MockInterface::new());
    display.set_vcomh_deselect(VcomhLevel1309::V078).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xDB, 0x34]));
}

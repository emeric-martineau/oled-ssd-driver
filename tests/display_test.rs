use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use oled_ssd_driver::{Ssd1306, HEIGHT, WIDTH};

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
fn test_dimensions() {
    let display = Ssd1306::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(128, 64));
}

#[test]
fn test_set_pixel_in_bounds() {
    let mut display = Ssd1306::new(MockInterface::new());
    display.set_pixel(0, 0, true);
    assert_eq!(display.buffer()[0], 0x01);

    display.set_pixel(0, 7, true);
    assert_eq!(display.buffer()[0], 0x81);

    display.set_pixel(127, 63, true);
    let idx = 7 * WIDTH as usize + 127;
    assert_eq!(display.buffer()[idx], 0x80);
}

#[test]
fn test_set_pixel_out_of_bounds() {
    let mut display = Ssd1306::new(MockInterface::new());
    display.set_pixel(128, 0, true);
    display.set_pixel(0, 64, true);
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_clear_buffer() {
    let mut display = Ssd1306::new(MockInterface::new());
    display.set_pixel(10, 10, true);
    display.clear_buffer();
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_draw_target_pixel() {
    let mut display = Ssd1306::new(MockInterface::new());
    let pixels = [
        Pixel(Point::new(0, 0), BinaryColor::On),
        Pixel(Point::new(1, 8), BinaryColor::On),
        Pixel(Point::new(-1, 0), BinaryColor::On), // out of bounds, ignored
    ];
    display.draw_iter(pixels).unwrap();

    assert_eq!(display.buffer()[0], 0x01);
    // x=1, y=8 → page 1, bit 0
    let idx = WIDTH as usize + 1;
    assert_eq!(display.buffer()[idx], 0x01);
}

#[test]
fn test_init_sends_commands() {
    let mut display = Ssd1306::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = &display.release();
    // Init should have sent multiple commands and no data
    assert!(!iface.commands.is_empty());
    assert!(iface.data.is_empty());
}

#[test]
fn test_flush_sends_data() {
    let mut display = Ssd1306::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_pixel(0, 0, true);
    display.flush().unwrap();

    let iface = display.release();
    assert!(!iface.data.is_empty());
    let total_data: usize = iface.data.iter().map(|d| d.len()).sum();
    assert_eq!(total_data, WIDTH as usize * (HEIGHT as usize / 8));
}

#[test]
fn test_set_contrast() {
    let mut display = Ssd1306::new(MockInterface::new());
    display.set_contrast(0xFF).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x81, 0xFF]));
}

#[test]
fn test_display_on_off() {
    let mut display = Ssd1306::new(MockInterface::new());
    display.display_on(false).unwrap();
    display.display_on(true).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAE]));
    assert!(iface.commands.iter().any(|c| c == &[0xAF]));
}

#[test]
fn test_set_inverse() {
    let mut display = Ssd1306::new(MockInterface::new());
    display.set_inverse(true).unwrap();
    display.set_inverse(false).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xA7]));
    assert!(iface.commands.iter().any(|c| c == &[0xA6]));
}

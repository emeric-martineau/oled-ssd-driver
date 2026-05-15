use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::Gray4,
    Pixel,
};
use oled_ssd_driver::Ssd1320;

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
fn test_1320_dimensions() {
    let display = Ssd1320::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(160, 160));
}

#[test]
fn test_1320_size_config() {
    let display = Ssd1320::new(MockInterface::new());
    assert_eq!(display.size_config().width, 160);
    assert_eq!(display.size_config().height, 160);
}

#[test]
fn test_1320_set_pixel_even_x() {
    let mut display = Ssd1320::new(MockInterface::new());
    // x=0 (even) → high nibble of byte 0
    display.set_pixel(0, 0, 0x0F);
    assert_eq!(display.buffer()[0], 0xF0);
}

#[test]
fn test_1320_set_pixel_odd_x() {
    let mut display = Ssd1320::new(MockInterface::new());
    // x=1 (odd) → low nibble of byte 0
    display.set_pixel(1, 0, 0x0A);
    assert_eq!(display.buffer()[0], 0x0A);
}

#[test]
fn test_1320_set_pixel_both_nibbles() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.set_pixel(0, 0, 0x0F);
    display.set_pixel(1, 0, 0x05);
    assert_eq!(display.buffer()[0], 0xF5);
}

#[test]
fn test_1320_set_pixel_row1() {
    let mut display = Ssd1320::new(MockInterface::new());
    // Row 1, x=0 → byte at offset 80 (160/2 columns per row)
    display.set_pixel(0, 1, 0x0C);
    assert_eq!(display.buffer()[80], 0xC0);
}

#[test]
fn test_1320_set_pixel_out_of_bounds() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.set_pixel(160, 0, 0x0F);
    display.set_pixel(0, 160, 0x0F);
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_1320_set_pixel_clamps_gray_value() {
    let mut display = Ssd1320::new(MockInterface::new());
    // Value > 0x0F should be masked to lower 4 bits
    display.set_pixel(0, 0, 0xFF);
    assert_eq!(display.buffer()[0], 0xF0);
}

#[test]
fn test_1320_clear_buffer() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.set_pixel(10, 10, 0x0F);
    display.clear_buffer();
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_1320_draw_target_pixel() {
    let mut display = Ssd1320::new(MockInterface::new());
    let pixels = [
        Pixel(Point::new(0, 0), Gray4::new(0x0F)),
        Pixel(Point::new(1, 0), Gray4::new(0x0A)),
        Pixel(Point::new(-1, 0), Gray4::new(0x0F)), // out of bounds, ignored
        Pixel(Point::new(0, 160), Gray4::new(0x0F)), // out of bounds, ignored
    ];
    display.draw_iter(pixels).unwrap();

    assert_eq!(display.buffer()[0], 0xFA);
}

#[test]
fn test_1320_init_sends_commands() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    assert!(!iface.commands.is_empty());
    assert!(iface.data.is_empty());
}

#[test]
fn test_1320_init_display_off_first() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    // First command should be display OFF (AEh)
    assert_eq!(iface.commands[0], vec![0xAE]);
}

#[test]
fn test_1320_init_multiplex_ratio() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    // Multiplex ratio should be height-1 = 159
    assert!(iface.commands.iter().any(|c| c == &[0xA8, 159]));
}

#[test]
fn test_1320_init_display_on_at_end() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    // Last command should be display ON (AFh)
    assert_eq!(iface.commands.last().unwrap(), &vec![0xAF]);
}

#[test]
fn test_1320_flush_sends_data() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_pixel(0, 0, 0x0F);
    display.flush().unwrap();

    let iface = display.release();
    assert!(!iface.data.is_empty());
    // 80 columns × 160 rows = 12800 bytes
    let total_data: usize = iface.data.iter().map(|d| d.len()).sum();
    assert_eq!(total_data, 80 * 160);
}

#[test]
fn test_1320_flush_sets_address_window() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.flush().unwrap();

    let iface = display.release();
    // Column address command (15h, 00h, 4Fh) — 0 to 79
    assert!(iface.commands.iter().any(|c| c == &[0x15, 0x00, 79]));
    // Row address command (75h, 00h, 9Fh) — 0 to 159
    assert!(iface.commands.iter().any(|c| c == &[0x75, 0x00, 159]));
}

#[test]
fn test_1320_set_contrast() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.set_contrast(0xFF).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x81, 0xFF]));
}

#[test]
fn test_1320_display_on_off() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.display_on(false).unwrap();
    display.display_on(true).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAE]));
    assert!(iface.commands.iter().any(|c| c == &[0xAF]));
}

#[test]
fn test_1320_set_inverse() {
    let mut display = Ssd1320::new(MockInterface::new());
    display.set_inverse(true).unwrap();
    display.set_inverse(false).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xA7]));
    assert!(iface.commands.iter().any(|c| c == &[0xA6]));
}

#[test]
fn test_1320_set_iref() {
    use oled_ssd_driver::IrefSetting1320;

    let mut display = Ssd1320::new(MockInterface::new());
    display.set_iref(IrefSetting1320::Internal).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x10]));
}

#[test]
fn test_1320_buffer_size() {
    let display = Ssd1320::new(MockInterface::new());
    // 160 pixels / 2 pixels per byte × 160 rows = 12800 bytes
    assert_eq!(display.buffer().len(), 12800);
}

#[test]
fn test_1320_last_pixel() {
    let mut display = Ssd1320::new(MockInterface::new());
    // Last pixel: x=159 (odd), y=159
    display.set_pixel(159, 159, 0x0B);
    let idx = 159 * 80 + 79; // row 159, column 79
    assert_eq!(display.buffer()[idx], 0x0B);
}

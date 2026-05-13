use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use oled_ssd_driver::{DisplaySize, Ssd1306, Ssd1306b, SIZE_128X32, SIZE_128X64, SIZE_96X16};

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
fn test_128x32_dimensions() {
    let display = Ssd1306::new_with_size(MockInterface::new(), SIZE_128X32);
    assert_eq!(display.size(), Size::new(128, 32));
}

#[test]
fn test_128x32_buffer_size() {
    let display = Ssd1306::new_with_size(MockInterface::new(), SIZE_128X32);
    // 128 * 4 pages = 512 bytes
    assert_eq!(display.buffer().len(), 512);
}

#[test]
fn test_128x32_pixel_bounds() {
    let mut display = Ssd1306::new_with_size(MockInterface::new(), SIZE_128X32);
    // y=31 should work (last row)
    display.set_pixel(0, 31, true);
    assert_eq!(display.buffer()[3 * 128], 0x80);

    // y=32 should be ignored (out of bounds)
    display.set_pixel(0, 32, true);
    // Buffer should not have changed beyond the bounds check
    assert_eq!(display.buffer()[3 * 128], 0x80);
}

#[test]
fn test_128x32_draw_target_clips() {
    let mut display = Ssd1306::new_with_size(MockInterface::new(), SIZE_128X32);
    let pixels = [
        Pixel(Point::new(0, 0), BinaryColor::On),
        Pixel(Point::new(0, 31), BinaryColor::On),
        Pixel(Point::new(0, 32), BinaryColor::On), // out of bounds
        Pixel(Point::new(127, 0), BinaryColor::On),
        Pixel(Point::new(128, 0), BinaryColor::On), // out of bounds
    ];
    display.draw_iter(pixels).unwrap();

    assert_eq!(display.buffer()[0], 0x01); // (0,0)
    assert_eq!(display.buffer()[3 * 128], 0x80); // (0,31) = page 3, bit 7
    assert_eq!(display.buffer()[127], 0x01); // (127,0)
}

#[test]
fn test_128x32_init_multiplex_ratio() {
    let mut display = Ssd1306::new_with_size(MockInterface::new(), SIZE_128X32);
    display.init().unwrap();

    let iface = display.release();
    // Multiplex ratio should be 31 (height - 1)
    assert!(iface.commands.iter().any(|c| c == &[0xA8, 31]));
}

#[test]
fn test_128x32_init_com_pin_sequential() {
    let mut display = Ssd1306::new_with_size(MockInterface::new(), SIZE_128X32);
    display.init().unwrap();

    let iface = display.release();
    // COM pin config for <=32 height: sequential (A[4]=0), no remap (A[5]=0) → 0x02
    assert!(iface.commands.iter().any(|c| c == &[0xDA, 0x02]));
}

#[test]
fn test_128x64_init_com_pin_alternative() {
    let mut display = Ssd1306::new_with_size(MockInterface::new(), SIZE_128X64);
    display.init().unwrap();

    let iface = display.release();
    // COM pin config for >32 height: alternative (A[4]=1), no remap (A[5]=0) → 0x12
    assert!(iface.commands.iter().any(|c| c == &[0xDA, 0x12]));
}

#[test]
fn test_128x32_flush_data_size() {
    let mut display = Ssd1306::new_with_size(MockInterface::new(), SIZE_128X32);
    display.init().unwrap();
    display.flush().unwrap();

    let iface = display.release();
    let total_data: usize = iface.data.iter().map(|d| d.len()).sum();
    assert_eq!(total_data, 128 * 4); // 512 bytes for 128x32
}

#[test]
fn test_96x16_dimensions() {
    let display = Ssd1306::new_with_size(MockInterface::new(), SIZE_96X16);
    assert_eq!(display.size(), Size::new(96, 16));
    assert_eq!(display.buffer().len(), 96 * 2); // 2 pages
}

#[test]
fn test_custom_size() {
    let custom = DisplaySize::new(64, 32);
    let display = Ssd1306::new_with_size(MockInterface::new(), custom);
    assert_eq!(display.size(), Size::new(64, 32));
    assert_eq!(display.buffer().len(), 64 * 4);
}

#[test]
fn test_ssd1306b_custom_size() {
    let mut display = Ssd1306b::new_with_size(MockInterface::new(), SIZE_128X32);
    assert_eq!(display.size(), Size::new(128, 32));

    display.init().unwrap();
    let iface = display.release();
    // Should have multiplex ratio 31
    assert!(iface.commands.iter().any(|c| c == &[0xA8, 31]));
    // Should still have IREF and charge pump
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x10]));
    assert!(iface.commands.iter().any(|c| c == &[0x8D, 0x14]));
}

#[test]
fn test_default_new_is_128x64() {
    let display = Ssd1306::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(128, 64));
    assert_eq!(display.buffer().len(), 1024);
}

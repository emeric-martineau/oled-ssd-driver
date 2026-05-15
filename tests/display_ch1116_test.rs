use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use oled_ssd_driver::{
    BreathingBrightnessCh1116, Ch1116, IrefResistorCh1116, IrefSourceCh1116, PumpVoltageCh1116,
    ScrollDirectionCh1116, ScrollIntervalCh1116, ScrollModeCh1116,
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
fn test_ch1116_dimensions() {
    let display = Ch1116::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(132, 64));
}

#[test]
fn test_ch1116_set_pixel() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_pixel(0, 0, true);
    assert_eq!(display.buffer()[0], 0x01);

    display.set_pixel(131, 63, true);
    let idx = 7 * 132 + 131;
    assert_eq!(display.buffer()[idx], 0x80);
}

#[test]
fn test_ch1116_set_pixel_out_of_bounds() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_pixel(132, 0, true);
    display.set_pixel(0, 64, true);
    // Buffer should remain all zeros
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_ch1116_init_sends_commands() {
    let mut display = Ch1116::new(MockInterface::new());
    display.init(&Default::default()).unwrap();

    let iface = display.release();
    // Should contain display OFF
    assert!(iface.commands.iter().any(|c| c == &[0xAE]));
    // Should contain DC-DC ON
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x8B]));
    // Should contain segment remap (reverse = A1)
    assert!(iface.commands.iter().any(|c| c == &[0xA1]));
    // Should contain COM scan reversed (C8)
    assert!(iface.commands.iter().any(|c| c == &[0xC8]));
    // Should contain pump voltage 7.4V (0x31)
    assert!(iface.commands.iter().any(|c| c == &[0x31]));
    // Should contain display ON
    assert!(iface.commands.iter().any(|c| c == &[0xAF]));
    // No data during init
    assert!(iface.data.is_empty());
}

#[test]
fn test_ch1116_init_with_custom_config() {
    use oled_ssd_driver::InitConfig;
    let mut display = Ch1116::new(MockInterface::new());
    let config = InitConfig {
        segment_remap: false,
        com_scan_reversed: false,
        display_on: true,
        ..Default::default()
    };
    display.init(&config).unwrap();

    let iface = display.release();
    // Segment normal (A0)
    assert!(iface.commands.iter().any(|c| c == &[0xA0]));
    // COM scan normal (C0)
    assert!(iface.commands.iter().any(|c| c == &[0xC0]));
}

#[test]
fn test_ch1116_flush_sends_page_mode_data() {
    let mut display = Ch1116::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_pixel(0, 0, true);
    display.flush().unwrap();

    let iface = display.release();
    // Should have 8 pages of data
    assert_eq!(iface.data.len(), 8);
    // Each page should be 132 bytes
    for page_data in &iface.data {
        assert_eq!(page_data.len(), 132);
    }
    // Total data = 132 * 8 = 1056
    let total_data: usize = iface.data.iter().map(|d| d.len()).sum();
    assert_eq!(total_data, 132 * 8);
    // First page should have pixel set at byte 0
    assert_eq!(iface.data[0][0], 0x01);
}

#[test]
fn test_ch1116_flush_page_addressing_commands() {
    let mut display = Ch1116::new(MockInterface::new());
    display.flush().unwrap();

    let iface = display.release();
    // Should have page address commands (B0-B7) and column commands (00, 10) for each page
    for page in 0u8..8 {
        assert!(iface.commands.iter().any(|c| c == &[0xB0 | page]));
    }
    // Lower column address 0x00 should appear 8 times
    let lower_col_count = iface
        .commands
        .iter()
        .filter(|c| c == &&vec![0x00u8])
        .count();
    assert_eq!(lower_col_count, 8);
    // Upper column address 0x10 should appear 8 times
    let upper_col_count = iface
        .commands
        .iter()
        .filter(|c| c == &&vec![0x10u8])
        .count();
    assert_eq!(upper_col_count, 8);
}

#[test]
fn test_ch1116_set_pump_voltage() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_pump_voltage(PumpVoltageCh1116::V9_0).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x32]));
}

#[test]
fn test_ch1116_set_pump_voltage_v10() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_pump_voltage(PumpVoltageCh1116::V10_0).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x33]));
}

#[test]
fn test_ch1116_set_pump_voltage_v6_4() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_pump_voltage(PumpVoltageCh1116::V6_4).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x30]));
}

#[test]
fn test_ch1116_set_iref_internal() {
    let mut display = Ch1116::new(MockInterface::new());
    display
        .set_iref(IrefSourceCh1116::Internal, IrefResistorCh1116::K310)
        .unwrap();

    let iface = display.release();
    // Internal (D=1 → bit2=0x04) + K310 (0x01) = 0x05
    assert!(iface.commands.iter().any(|c| c == &[0x82, 0x05]));
}

#[test]
fn test_ch1116_set_iref_external() {
    let mut display = Ch1116::new(MockInterface::new());
    display
        .set_iref(IrefSourceCh1116::External, IrefResistorCh1116::K510)
        .unwrap();

    let iface = display.release();
    // External (D=0 → 0x00) + K510 (0x00) = 0x00
    assert!(iface.commands.iter().any(|c| c == &[0x82, 0x00]));
}

#[test]
fn test_ch1116_set_vcom_deselect() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_vcom_deselect(0x35).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xDB, 0x35]));
}

#[test]
fn test_ch1116_set_dc_dc_on() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_dc_dc(true).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x8B]));
}

#[test]
fn test_ch1116_set_dc_dc_off() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_dc_dc(false).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAD, 0x8A]));
}

#[test]
fn test_ch1116_set_breathing_light() {
    let mut display = Ch1116::new(MockInterface::new());
    display
        .set_breathing_light(true, BreathingBrightnessCh1116::Half, 3)
        .unwrap();

    let iface = display.release();
    // ON=0x80, Half=0x08, interval=3 → 0x80 | 0x08 | 0x03 = 0x8B
    assert!(iface.commands.iter().any(|c| c == &[0x23, 0x8B]));
}

#[test]
fn test_ch1116_set_breathing_light_off() {
    let mut display = Ch1116::new(MockInterface::new());
    display
        .set_breathing_light(false, BreathingBrightnessCh1116::Full, 0)
        .unwrap();

    let iface = display.release();
    // OFF=0x00, Full=0x00, interval=0 → 0x00
    assert!(iface.commands.iter().any(|c| c == &[0x23, 0x00]));
}

#[test]
fn test_ch1116_horizontal_scroll() {
    let mut display = Ch1116::new(MockInterface::new());
    display
        .start_horizontal_scroll(
            ScrollDirectionCh1116::Right,
            0,
            7,
            ScrollIntervalCh1116::Frames6,
        )
        .unwrap();

    let iface = display.release();
    // Should deactivate first
    assert!(iface.commands.iter().any(|c| c == &[0x2E]));
    // Scroll setup: 0x26, start_page=0, interval=0, end_page=7
    assert!(iface
        .commands
        .iter()
        .any(|c| c == &[0x26, 0x00, 0x00, 0x07]));
    // Activate
    assert!(iface.commands.iter().any(|c| c == &[0x2F]));
}

#[test]
fn test_ch1116_horizontal_scroll_left() {
    let mut display = Ch1116::new(MockInterface::new());
    display
        .start_horizontal_scroll(
            ScrollDirectionCh1116::Left,
            1,
            5,
            ScrollIntervalCh1116::Frames32,
        )
        .unwrap();

    let iface = display.release();
    assert!(iface
        .commands
        .iter()
        .any(|c| c == &[0x27, 0x01, 0x01, 0x05]));
}

#[test]
fn test_ch1116_horizontal_scroll_invalid_pages() {
    let mut display = Ch1116::new(MockInterface::new());
    let result = display.start_horizontal_scroll(
        ScrollDirectionCh1116::Right,
        5,
        3,
        ScrollIntervalCh1116::Frames6,
    );
    assert_eq!(result, Err(oled_ssd_driver::DisplayError::InvalidArgument));
}

#[test]
fn test_ch1116_set_scroll_columns() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_scroll_columns(10, 100).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x24, 10, 100]));
}

#[test]
fn test_ch1116_set_scroll_columns_invalid() {
    let mut display = Ch1116::new(MockInterface::new());
    let result = display.set_scroll_columns(100, 50);
    assert_eq!(result, Err(oled_ssd_driver::DisplayError::InvalidArgument));
}

#[test]
fn test_ch1116_set_scroll_mode() {
    let mut display = Ch1116::new(MockInterface::new());
    display
        .set_scroll_mode(ScrollModeCh1116::SingleScreen)
        .unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x2D]));
}

#[test]
fn test_ch1116_stop_scroll() {
    let mut display = Ch1116::new(MockInterface::new());
    display.stop_scroll().unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x2E]));
}

#[test]
fn test_ch1116_draw_target_pixel() {
    let mut display = Ch1116::new(MockInterface::new());
    let pixels = [
        Pixel(Point::new(0, 0), BinaryColor::On),
        Pixel(Point::new(1, 8), BinaryColor::On),
        Pixel(Point::new(-1, 0), BinaryColor::On), // out of bounds, ignored
        Pixel(Point::new(131, 63), BinaryColor::On),
    ];
    display.draw_iter(pixels).unwrap();

    assert_eq!(display.buffer()[0], 0x01);
    // (1, 8) → page 1, x=1
    let idx = 132 + 1;
    assert_eq!(display.buffer()[idx], 0x01);
    // (131, 63) → page 7, x=131
    let idx = 7 * 132 + 131;
    assert_eq!(display.buffer()[idx], 0x80);
}

#[test]
fn test_ch1116_clear_buffer() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_pixel(50, 30, true);
    display.clear_buffer();
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_ch1116_set_contrast() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_contrast(0xFF).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0x81, 0xFF]));
}

#[test]
fn test_ch1116_set_inverse() {
    let mut display = Ch1116::new(MockInterface::new());
    display.set_inverse(true).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xA7]));
}

#[test]
fn test_ch1116_display_on_off() {
    let mut display = Ch1116::new(MockInterface::new());
    display.display_on(false).unwrap();

    let iface = display.release();
    assert!(iface.commands.iter().any(|c| c == &[0xAE]));
}

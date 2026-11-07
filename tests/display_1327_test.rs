use embedded_graphics_core::{
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::Gray4,
    Drawable, Pixel,
};
use oled_ssd_driver::{ssd1327::DEFAULT_DISPLAY_SIZE, InitConfig1327, Ssd1327};

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
fn test_1327_dimensions() {
    let display = Ssd1327::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(128, 128));
}

#[test]
fn test_1327_size_config() {
    let display = Ssd1327::new(MockInterface::new());
    assert_eq!(display.size_config(), DEFAULT_DISPLAY_SIZE);
    assert_eq!(display.size_config().width, 128);
    assert_eq!(display.size_config().height, 128);
}

#[test]
fn test_1327_set_pixel_even_x() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.set_pixel(0, 0, 0x0A);
    assert_eq!(display.buffer()[0], 0xA0);
}

#[test]
fn test_1327_set_pixel_odd_x() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.set_pixel(1, 0, 0x05);
    assert_eq!(display.buffer()[0], 0x05);
}

#[test]
fn test_1327_set_pixel_both_nibbles() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.set_pixel(0, 0, 0x0F);
    display.set_pixel(1, 0, 0x03);
    assert_eq!(display.buffer()[0], 0xF3);
}

#[test]
fn test_1327_set_pixel_row1() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.set_pixel(0, 1, 0x07);
    // Row 1 starts at offset 64 (columns = 64)
    assert_eq!(display.buffer()[64], 0x70);
}

#[test]
fn test_1327_set_pixel_out_of_bounds() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.set_pixel(128, 0, 0x0F);
    display.set_pixel(0, 128, 0x0F);
    display.set_pixel(255, 255, 0x0F);
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_1327_set_pixel_clamps_gray_value() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.set_pixel(0, 0, 0xFF);
    // Only lower 4 bits: 0xFF & 0x0F = 0x0F
    assert_eq!(display.buffer()[0], 0xF0);
}

#[test]
fn test_1327_clear_buffer() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.set_pixel(10, 10, 0x0F);
    display.clear_buffer();
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_1327_draw_target_pixel() {
    let mut display = Ssd1327::new(MockInterface::new());
    Pixel(Point::new(2, 0), Gray4::new(0x0C))
        .draw(&mut display)
        .unwrap();
    // x=2 is even, column index 1, high nibble
    assert_eq!(display.buffer()[1], 0xC0);
}

#[test]
fn test_1327_init_sends_commands() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    // First command: display OFF (0xAE)
    assert_eq!(cmds[0], vec![0xAE]);
}

#[test]
fn test_1327_init_display_on_at_end() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xAF]);
}

#[test]
fn test_1327_init_multiplex_ratio() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    let mux = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xA8).unwrap();
    assert_eq!(mux[1], 127); // height - 1
}

#[test]
fn test_1327_init_remap() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    let remap = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xA0).unwrap();
    assert_eq!(remap[1], 0x51); // column remap, COM remap, and COM split enabled
}

#[test]
fn test_1327_init_function_a_internal_vdd() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    let func_a = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xAB).unwrap();
    assert_eq!(func_a[1], 0x01); // internal VDD enabled
}

#[test]
fn test_1327_init_function_a_external_vdd() {
    let config = InitConfig1327 {
        internal_vdd: false,
        ..Default::default()
    };
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&config).unwrap();
    let cmds = &display.release().commands;
    let func_a = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xAB).unwrap();
    assert_eq!(func_a[1], 0x00); // internal VDD disabled
}

#[test]
fn test_1327_init_linear_lut() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    assert!(cmds.iter().any(|c| c == &vec![0xB9]));
}

#[test]
fn test_1327_flush_sends_data() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_pixel(0, 0, 0x0F);
    display.flush().unwrap();
    let iface = display.release();
    let data = &iface.data;
    assert_eq!(data.len(), 1);
    assert_eq!(data[0].len(), 8192); // 64*128
}

#[test]
fn test_1327_flush_sets_address_window() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.flush().unwrap();
    let iface = display.release();
    let cmds = &iface.commands;
    // Find column address command (0x15)
    let col_cmd = cmds.iter().find(|c| c.len() >= 3 && c[0] == 0x15).unwrap();
    assert_eq!(col_cmd, &vec![0x15, 0x00, 63]); // start=0, end=63

    // Find row address command (0x75)
    let row_cmd = cmds.iter().find(|c| c.len() >= 3 && c[0] == 0x75).unwrap();
    assert_eq!(row_cmd, &vec![0x75, 0x00, 127]); // start=0, end=127
}

#[test]
fn test_1327_set_contrast() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_contrast(0xCF).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0x81, 0xCF]);
}

#[test]
fn test_1327_display_on_off() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.display_on(false).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xAE]);
}

#[test]
fn test_1327_set_inverse() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_inverse(true).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xA7]);
}

#[test]
fn test_1327_set_vcomh() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_vcomh(0x07).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xBE, 0x07]);
}

#[test]
fn test_1327_set_vcomh_clamps() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_vcomh(0xFF).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xBE, 0x07]); // masked to 3 bits
}

#[test]
fn test_1327_set_precharge_voltage() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_precharge_voltage(0x08).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xBC, 0x08]);
}

#[test]
fn test_1327_set_internal_vdd() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_internal_vdd(false).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xAB, 0x00]);
}

#[test]
fn test_1327_buffer_size() {
    let display = Ssd1327::new(MockInterface::new());
    assert_eq!(display.buffer().len(), 8192);
}

#[test]
fn test_1327_last_pixel() {
    let mut display = Ssd1327::new(MockInterface::new());
    display.set_pixel(127, 127, 0x0F);
    // x=127 odd, row=127, col=63 → idx = 127*64 + 63 = 8191
    assert_eq!(display.buffer()[8191], 0x0F);
}

#[test]
fn test_1327_init_with_remap_options() {
    let config = InitConfig1327 {
        column_remap: true,
        nibble_remap: true,
        vertical_address_increment: true,
        com_remap: true,
        com_split_odd_even: true,
        ..Default::default()
    };
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&config).unwrap();
    let cmds = &display.release().commands;
    let remap = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xA0).unwrap();
    // bit0=1, bit1=1, bit2=1, bit4=1, bit6=1 = 0x57
    assert_eq!(remap[1], 0x57);
}

#[test]
fn test_1327_init_function_b() {
    let config = InitConfig1327 {
        second_precharge_enable: true,
        external_vsl: true,
        ..Default::default()
    };
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&config).unwrap();
    let cmds = &display.release().commands;
    let func_b = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xD5).unwrap();
    assert_eq!(func_b[1], 0x03); // bit0=1 (ext VSL), bit1=1 (2nd precharge)
}

#[test]
fn test_1327_init_no_display_on() {
    let config = InitConfig1327 {
        display_on: false,
        ..Default::default()
    };
    let mut display = Ssd1327::new(MockInterface::new());
    display.init(&config).unwrap();
    let cmds = &display.release().commands;
    // Last command should NOT be 0xAF
    let last = cmds.last().unwrap();
    assert_ne!(last, &vec![0xAF]);
}

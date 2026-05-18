use embedded_graphics_core::{
    geometry::{OriginDimensions, Point, Size},
    pixelcolor::Gray4,
    Drawable, Pixel,
};
use oled_ssd_driver::{CurrentRange1325, InitConfig1325, Ssd1325, SIZE_128X80};

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
fn test_1325_dimensions() {
    let display = Ssd1325::new(MockInterface::new());
    assert_eq!(display.size(), Size::new(128, 80));
}

#[test]
fn test_1325_size_config() {
    let display = Ssd1325::new(MockInterface::new());
    assert_eq!(display.size_config(), SIZE_128X80);
    assert_eq!(display.size_config().width, 128);
    assert_eq!(display.size_config().height, 80);
}

#[test]
fn test_1325_set_pixel_even_x() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.set_pixel(0, 0, 0x0A);
    assert_eq!(display.buffer()[0], 0xA0);
}

#[test]
fn test_1325_set_pixel_odd_x() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.set_pixel(1, 0, 0x05);
    assert_eq!(display.buffer()[0], 0x05);
}

#[test]
fn test_1325_set_pixel_both_nibbles() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.set_pixel(0, 0, 0x0F);
    display.set_pixel(1, 0, 0x03);
    assert_eq!(display.buffer()[0], 0xF3);
}

#[test]
fn test_1325_set_pixel_row1() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.set_pixel(0, 1, 0x07);
    // Row 1 starts at offset 64 (columns = 64)
    assert_eq!(display.buffer()[64], 0x70);
}

#[test]
fn test_1325_set_pixel_out_of_bounds() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.set_pixel(128, 0, 0x0F);
    display.set_pixel(0, 80, 0x0F);
    display.set_pixel(255, 255, 0x0F);
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_1325_set_pixel_clamps_gray_value() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.set_pixel(0, 0, 0xFF);
    // Only lower 4 bits: 0xFF & 0x0F = 0x0F
    assert_eq!(display.buffer()[0], 0xF0);
}

#[test]
fn test_1325_clear_buffer() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.set_pixel(10, 10, 0x0F);
    display.clear_buffer();
    assert!(display.buffer().iter().all(|&b| b == 0));
}

#[test]
fn test_1325_draw_target_pixel() {
    let mut display = Ssd1325::new(MockInterface::new());
    Pixel(Point::new(2, 0), Gray4::new(0x0C))
        .draw(&mut display)
        .unwrap();
    // x=2 is even, column index 1, high nibble
    assert_eq!(display.buffer()[1], 0xC0);
}

#[test]
fn test_1325_init_sends_commands() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    // First command: display OFF (0xAE)
    assert_eq!(cmds[0], vec![0xAE]);
}

#[test]
fn test_1325_init_display_off_first() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    assert_eq!(cmds[0], vec![0xAE]);
}

#[test]
fn test_1325_init_display_on_at_end() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xAF]);
}

#[test]
fn test_1325_init_multiplex_ratio() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    let mux = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xA8).unwrap();
    assert_eq!(mux[1], 79); // height - 1
}

#[test]
fn test_1325_init_remap() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    let remap = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xA0).unwrap();
    assert_eq!(remap[1], 0x00); // all disabled by default
}

#[test]
fn test_1325_init_master_config() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    let cmds = &display.release().commands;
    let master = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xAD).unwrap();
    assert_eq!(master[1], 0x02); // external VCC, bit0=0
}

#[test]
fn test_1325_flush_sends_data() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_pixel(0, 0, 0x0F);
    display.flush().unwrap();
    let iface = display.release();
    let data = &iface.data;
    assert_eq!(data.len(), 1);
    assert_eq!(data[0].len(), 5120); // 64*80
}

#[test]
fn test_1325_flush_sets_address_window() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.flush().unwrap();
    let iface = display.release();
    let cmds = &iface.commands;
    // Find column address command (0x15)
    let col_cmd = cmds.iter().find(|c| c.len() >= 3 && c[0] == 0x15).unwrap();
    assert_eq!(col_cmd, &vec![0x15, 0x00, 63]); // start=0, end=63

    // Find row address command (0x75)
    let row_cmd = cmds.iter().find(|c| c.len() >= 3 && c[0] == 0x75).unwrap();
    assert_eq!(row_cmd, &vec![0x75, 0x00, 79]); // start=0, end=79
}

#[test]
fn test_1325_set_contrast() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_contrast(0x3F).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0x81, 0x3F]);
}

#[test]
fn test_1325_display_on_off() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.display_on(false).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xAE]);
}

#[test]
fn test_1325_set_inverse() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_inverse(true).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xA7]);
}

#[test]
fn test_1325_set_current_range_full() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_current_range(CurrentRange1325::Full).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0x87]);
}

#[test]
fn test_1325_set_current_range_half() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_current_range(CurrentRange1325::Half).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0x85]);
}

#[test]
fn test_1325_set_vcomh() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_vcomh(0x1D).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xBE, 0x1D]);
}

#[test]
fn test_1325_set_precharge_voltage() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&Default::default()).unwrap();
    display.set_precharge_voltage(0x20).unwrap();
    let cmds = &display.release().commands;
    let last = cmds.last().unwrap();
    assert_eq!(last, &vec![0xBC, 0x20]);
}

#[test]
fn test_1325_buffer_size() {
    let display = Ssd1325::new(MockInterface::new());
    assert_eq!(display.buffer().len(), 5120);
}

#[test]
fn test_1325_last_pixel() {
    let mut display = Ssd1325::new(MockInterface::new());
    display.set_pixel(127, 79, 0x0F);
    // x=127 odd, row=79, col=63 → idx = 79*64 + 63 = 5119
    assert_eq!(display.buffer()[5119], 0x0F);
}

#[test]
fn test_1325_init_with_remap_options() {
    let config = InitConfig1325 {
        column_remap: true,
        nibble_remap: true,
        vertical_address_increment: true,
        com_remap: true,
        com_split_odd_even: true,
        ..Default::default()
    };
    let mut display = Ssd1325::new(MockInterface::new());
    display.init(&config).unwrap();
    let cmds = &display.release().commands;
    let remap = cmds.iter().find(|c| c.len() >= 2 && c[0] == 0xA0).unwrap();
    // bit0=1, bit1=1, bit2=1, bit4=1, bit6=1 = 0x57
    assert_eq!(remap[1], 0x57);
}

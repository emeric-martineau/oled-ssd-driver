# Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
oled-ssd-driver = "0.1"
```

## SSD0303 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd0303, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::BinaryColor, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd0303::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD0303: 132x64 monochrome, page-mode addressing, internal DC-DC converter
let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1300 over SPI

```rust
use oled_ssd_driver::{SpiInterface, Ssd1300};
use embedded_graphics::{prelude::*, pixelcolor::BinaryColor, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1300::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1300: 104x48 monochrome, page-mode addressing, internal DC-DC booster
let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1306 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1306, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::BinaryColor, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

// Construct the interface (pass your platform's I2C peripheral)
let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1306::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1306B over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1306b, ChargePumpVoltage, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1306b::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1306B-specific: select charge pump voltage
display.set_charge_pump_voltage(ChargePumpVoltage::V8_5).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

## SSD1307 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1307, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1307::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1307: no charge pump — requires external VCC (7-15V)
// ... draw with embedded-graphics ...

display.flush().unwrap();
```

## SSD1309 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1309, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1309::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1309: no charge pump — requires external VCC (7-18V)
// ... draw with embedded-graphics ...

display.flush().unwrap();
```

## SSD1312 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1312, ChargePumpVoltage1312, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1312::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1312-specific: select charge pump voltage (7.5V, 8.5V, 9.0V, or 10.0V)
display.set_charge_pump_voltage(ChargePumpVoltage1312::V8_5).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

## SSD1315 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ssd1315, ChargePumpVoltage1315, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1315::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1315-specific: select charge pump voltage (7.5V, 8.5V, or 9.0V)
display.set_charge_pump_voltage(ChargePumpVoltage1315::V8_5).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

## CH1116 over I2C

```rust
use oled_ssd_driver::{I2cInterface, Ch1116, PumpVoltageCh1116, DEFAULT_I2C_ADDRESS};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ch1116::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// CH1116-specific: select pump voltage (6.4V, 7.4V, 9.0V, or 10.0V)
display.set_pump_voltage(PumpVoltageCh1116::V9_0).unwrap();

// ... draw with embedded-graphics ...

display.flush().unwrap();
```

## SSD1320 over I2C (Grayscale)

```rust
use oled_ssd_driver::{I2cInterface, Ssd1320, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::Gray4, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1320::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1320 uses Gray4 (4-bit grayscale, 16 levels: 0x00-0x0F)
let style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1325 over I2C (Grayscale)

```rust
use oled_ssd_driver::{I2cInterface, Ssd1325, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::Gray4, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1325::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1325 uses Gray4 (4-bit grayscale, 16 levels: 0x00-0x0F)
let style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1327 over I2C (Grayscale)

```rust
use oled_ssd_driver::{I2cInterface, Ssd1327, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::Gray4, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1327::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1327 uses Gray4 (4-bit grayscale, 16 levels: 0x00-0x0F)
let style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1362 over I2C (Grayscale)

```rust
use oled_ssd_driver::{I2cInterface, Ssd1362, DEFAULT_I2C_ADDRESS};
use embedded_graphics::{prelude::*, pixelcolor::Gray4, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = I2cInterface::new(i2c, DEFAULT_I2C_ADDRESS);
let mut display = Ssd1362::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1362 uses Gray4 (4-bit grayscale, 16 levels: 0x00-0x0F)
let style = MonoTextStyle::new(&FONT_6X10, Gray4::new(0x0F));
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1331 over SPI (Color)

```rust
use oled_ssd_driver::{SpiInterface, Ssd1331};
use embedded_graphics::{prelude::*, pixelcolor::Rgb565, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1331::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1331 uses Rgb565 (65K color, 16 bits per pixel)
let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1332 over SPI (Color)

```rust
use oled_ssd_driver::{SpiInterface, Ssd1332};
use embedded_graphics::{prelude::*, pixelcolor::Rgb565, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1332::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1332 uses Rgb565 (65K color, 16 bits per pixel)
let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SSD1351 over SPI (Color)

```rust
use oled_ssd_driver::{SpiInterface, Ssd1351};
use embedded_graphics::{prelude::*, pixelcolor::Rgb565, text::Text,
    mono_font::{ascii::FONT_6X10, MonoTextStyle}};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1351::new(interface);

display.init(&Default::default()).unwrap();
display.clear_buffer();

// SSD1351 uses Rgb565 (65K color, 16 bits per pixel)
let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
Text::new("Hello!", Point::new(0, 10), style)
    .draw(&mut display)
    .unwrap();

display.flush().unwrap();
```

## SPI Interface

```rust
use oled_ssd_driver::{SpiInterface, Ssd1306};

let interface = SpiInterface::new(spi_device, dc_pin);
let mut display = Ssd1306::new(interface);
display.init(&Default::default()).unwrap();
```

## Async Usage

```rust
let mut display = Ssd1306::new(interface);
display.init_async(&Default::default()).await.unwrap();
display.flush_async().await.unwrap();
```

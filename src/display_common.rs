use crate::command::{AddressingMode, Command, VcomhLevel};
use crate::error::DisplayError;
use crate::interface::{AsyncDisplayInterface, DisplayInterface};
use crate::{HEIGHT, PAGES, WIDTH};

pub(crate) const BUFFER_SIZE: usize = WIDTH as usize * PAGES as usize;

pub(crate) struct DisplayCore<DI> {
    pub interface: DI,
    pub buffer: [u8; BUFFER_SIZE],
}

impl<DI> DisplayCore<DI> {
    pub fn new(interface: DI) -> Self {
        Self {
            interface,
            buffer: [0u8; BUFFER_SIZE],
        }
    }

    pub fn set_pixel(&mut self, x: u8, y: u8, on: bool) {
        if x >= WIDTH || y >= HEIGHT {
            return;
        }
        let page = (y / 8) as usize;
        let bit = y % 8;
        let idx = page * WIDTH as usize + x as usize;
        if on {
            self.buffer[idx] |= 1 << bit;
        } else {
            self.buffer[idx] &= !(1 << bit);
        }
    }

    pub fn clear_buffer(&mut self) {
        self.buffer = [0u8; BUFFER_SIZE];
    }
}

impl<DI: DisplayInterface> DisplayCore<DI> {
    pub fn send_command(&mut self, cmd: Command) -> Result<(), DisplayError> {
        let (bytes, len) = cmd.bytes();
        self.interface.send_command(&bytes[..len])
    }

    pub fn flush(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::SetColumnAddress {
            start: 0,
            end: WIDTH - 1,
        })?;
        self.send_command(Command::SetPageAddress {
            start: 0,
            end: PAGES - 1,
        })?;
        self.interface.send_data(&self.buffer)
    }

    pub fn display_on(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOn(on))
    }

    pub fn set_contrast(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_command(Command::SetContrast(contrast))
    }

    pub fn set_inverse(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_command(Command::InverseDisplay(inverse))
    }

    pub fn stop_scroll(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DeactivateScroll)
    }

    pub fn init_common(&mut self) -> Result<(), DisplayError> {
        self.send_command(Command::DisplayOn(false))?;
        self.send_command(Command::SetDisplayClock {
            divide_ratio: 0,
            osc_freq: 0x8,
        })?;
        self.send_command(Command::SetMultiplexRatio(63))?;
        self.send_command(Command::SetDisplayOffset(0))?;
        self.send_command(Command::SetDisplayStartLine(0))?;
        self.send_command(Command::SetAddressingMode(AddressingMode::Horizontal))?;
        self.send_command(Command::SetSegmentRemap(true))?;
        self.send_command(Command::SetComScanDirection(true))?;
        self.send_command(Command::SetComPinConfig(
            crate::command::ComPinConfig::default(),
        ))?;
        self.send_command(Command::SetContrast(0x7F))?;
        self.send_command(Command::SetPreChargePeriod {
            phase1: 0x1,
            phase2: 0xF,
        })?;
        self.send_command(Command::SetVcomhDeselect(VcomhLevel::V077))?;
        self.send_command(Command::EntireDisplayOn(false))?;
        self.send_command(Command::InverseDisplay(false))?;
        self.send_command(Command::DeactivateScroll)?;
        Ok(())
    }
}

impl<DI: AsyncDisplayInterface> DisplayCore<DI> {
    pub async fn send_command_async(&mut self, cmd: Command) -> Result<(), DisplayError> {
        let (bytes, len) = cmd.bytes();
        self.interface.send_command(&bytes[..len]).await
    }

    pub async fn flush_async(&mut self) -> Result<(), DisplayError> {
        self.send_command_async(Command::SetColumnAddress {
            start: 0,
            end: WIDTH - 1,
        })
        .await?;
        self.send_command_async(Command::SetPageAddress {
            start: 0,
            end: PAGES - 1,
        })
        .await?;
        self.interface.send_data(&self.buffer).await
    }

    pub async fn display_on_async(&mut self, on: bool) -> Result<(), DisplayError> {
        self.send_command_async(Command::DisplayOn(on)).await
    }

    pub async fn set_contrast_async(&mut self, contrast: u8) -> Result<(), DisplayError> {
        self.send_command_async(Command::SetContrast(contrast))
            .await
    }

    pub async fn set_inverse_async(&mut self, inverse: bool) -> Result<(), DisplayError> {
        self.send_command_async(Command::InverseDisplay(inverse))
            .await
    }

    pub async fn init_common_async(&mut self) -> Result<(), DisplayError> {
        self.send_command_async(Command::DisplayOn(false)).await?;
        self.send_command_async(Command::SetDisplayClock {
            divide_ratio: 0,
            osc_freq: 0x8,
        })
        .await?;
        self.send_command_async(Command::SetMultiplexRatio(63))
            .await?;
        self.send_command_async(Command::SetDisplayOffset(0))
            .await?;
        self.send_command_async(Command::SetDisplayStartLine(0))
            .await?;
        self.send_command_async(Command::SetAddressingMode(AddressingMode::Horizontal))
            .await?;
        self.send_command_async(Command::SetSegmentRemap(true))
            .await?;
        self.send_command_async(Command::SetComScanDirection(true))
            .await?;
        self.send_command_async(Command::SetComPinConfig(
            crate::command::ComPinConfig::default(),
        ))
        .await?;
        self.send_command_async(Command::SetContrast(0x7F)).await?;
        self.send_command_async(Command::SetPreChargePeriod {
            phase1: 0x1,
            phase2: 0xF,
        })
        .await?;
        self.send_command_async(Command::SetVcomhDeselect(VcomhLevel::V077))
            .await?;
        self.send_command_async(Command::EntireDisplayOn(false))
            .await?;
        self.send_command_async(Command::InverseDisplay(false))
            .await?;
        self.send_command_async(Command::DeactivateScroll).await?;
        Ok(())
    }
}

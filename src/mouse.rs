use serde::Deserialize;
use std::error::Error;
use uinput::event::controller::Mouse;

pub struct MouseDevice {
    device: uinput::Device,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
}

impl MouseButton {
    fn to_uinput(self) -> Mouse {
        match self {
            MouseButton::Left => Mouse::Left,
            MouseButton::Right => Mouse::Right,
        }
    }
}

impl MouseDevice {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // funky permission issues when using uinput
        let device = uinput::open("/dev/uinput")?
            .name("rust-virtual-mouse")?
            .event(Mouse::Left)?
            .event(Mouse::Right)?
            .create()?;
        Ok(MouseDevice { device })
    }

    pub fn hold(&mut self, button: &MouseButton) -> Result<(), Box<dyn Error>> {
        self.device.send(button.to_uinput(), 1)?;
        self.device.synchronize()?;
        Ok(())
    }

    pub fn release(&mut self, button: &MouseButton) -> Result<(), Box<dyn Error>> {
        self.device.send(button.to_uinput(), 0)?;
        self.device.synchronize()?;
        Ok(())
    }

    pub fn click(&mut self, button: &MouseButton) -> Result<(), Box<dyn Error>> {
        self.hold(button)?;
        self.release(button)?;
        Ok(())
    }
}

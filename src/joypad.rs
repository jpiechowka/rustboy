pub enum JoypadButton {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

pub struct Joypad {
    p14: u8,
    p15: u8,
    mask: u8,
    pub is_interrupted: bool,
}

impl Default for Joypad {
    fn default() -> Self {
        Self::new()
    }
}

impl Joypad {
    pub fn new() -> Self {
        Self {
            p14: 0x0F,
            p15: 0x0F,
            mask: 0x30,
            is_interrupted: false,
        }
    }

    pub fn press_key(&mut self, pressed_button: JoypadButton) {
        match pressed_button {
            JoypadButton::Right => self.p14 &= !0x01,
            JoypadButton::Left => self.p14 &= !0x02,
            JoypadButton::Up => self.p14 &= !0x04,
            JoypadButton::Down => self.p14 &= !0x08,
            JoypadButton::A => self.p15 &= !0x01,
            JoypadButton::B => self.p15 &= !0x02,
            JoypadButton::Select => self.p15 &= !0x04,
            JoypadButton::Start => self.p15 &= !0x08,
        }
        self.is_interrupted = true;
    }

    pub fn release_key(&mut self, released_key: JoypadButton) {
        match released_key {
            JoypadButton::Right => self.p14 |= 0x01,
            JoypadButton::Left => self.p14 |= 0x02,
            JoypadButton::Up => self.p14 |= 0x04,
            JoypadButton::Down => self.p14 |= 0x08,
            JoypadButton::A => self.p15 |= 0x01,
            JoypadButton::B => self.p15 |= 0x02,
            JoypadButton::Select => self.p15 |= 0x04,
            JoypadButton::Start => self.p15 |= 0x08,
        }
    }
}

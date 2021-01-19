use bitfield::bitfield;

bitfield! {
    pub struct JoypadButtons(u8);

    impl Debug;

    pub button_a, set_button_a: 0;
    pub button_b, set_button_b: 1;
    pub select, set_select: 2;
    pub start, set_start: 3;
    pub up, set_up: 4;
    pub down, set_down: 5;
    pub left, set_left: 6;
    pub right, set_right: 7;
    pub get, set: 7, 0;
}

pub struct Joypad {
    strobe: bool,
    idx: u8,
    pub buttons: JoypadButtons,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            strobe: false,
            idx: 0,
            buttons: JoypadButtons(0),
        }
    }

    // Read the current value of the referenced button
    pub fn read(&mut self) -> u8 {
        if self.idx > 7 {
            return 1;
        }

        let response = (self.buttons.0 >> self.idx) & 1;

        if !self.strobe && self.idx < 8 {
            self.idx += 1;
        }

        response
    }

    pub fn write(&mut self, data: u8) {
        self.strobe = data & 0x1 != 0;

        if self.strobe {
            self.idx = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_strobe() {
        let mut joypad = Joypad::new();

        joypad.buttons.set(0b1001_1001);

        joypad.write(0);

        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 0);
        assert_eq!(joypad.read(), 0);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 0);
        assert_eq!(joypad.read(), 0);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);

        joypad.write(1);

        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
        assert_eq!(joypad.read(), 1);
    }
}

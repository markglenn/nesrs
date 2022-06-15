use std::fmt::Debug;

use bitfield::bitfield;

bitfield! {
    pub struct StatusRegister(u8);

    pub sprite_overflow, set_sprite_overflow: 5;
    pub sprite_0_hit, set_sprite_0_hit: 6;
    pub vblank, set_vblank: 7;
    pub load, _: 7, 0;
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister(0b1000_0000)
    }

    pub fn read(&mut self) -> u8 {
        let value = self.load();
        self.set_vblank(false);

        value
    }
}

impl Debug for StatusRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}", self.load())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_vblank() {
        let mut status = StatusRegister::new();

        assert_eq!(0b1000_0000, status.load());

        status.set_vblank(false);
        assert_eq!(0b0000_0000, status.load());
        status.set_vblank(true);
        assert_eq!(0b1000_0000, status.load());
    }

    #[test]
    fn set_sprite_0_hit() {
        let mut status = StatusRegister::new();

        status.set_sprite_0_hit(true);
        assert_eq!(0b1100_0000, status.load());
        status.set_sprite_0_hit(false);
        assert_eq!(0b1000_0000, status.load());
    }

    #[test]
    fn set_sprite_overflow() {
        let mut status = StatusRegister::new();

        status.set_sprite_overflow(true);
        assert_eq!(0b1010_0000, status.load());
        status.set_sprite_overflow(false);
        assert_eq!(0b1000_0000, status.load());
    }
}

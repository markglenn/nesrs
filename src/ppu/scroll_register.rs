use crate::hardware::register::Register;

pub struct ScrollRegister {
    pub x: Register<u8>,
    pub y: Register<u8>,
    latch: bool,
}

impl ScrollRegister {
    pub fn new() -> Self {
        ScrollRegister {
            x: Register::<u8>::new(),
            y: Register::<u8>::new(),
            latch: false,
        }
    }

    // Return the fine X scroll position
    // Refer to http://wiki.nesdev.com/w/index.php/PPU_scrolling
    pub fn fine_x_scroll(&self) -> u8 {
        self.x.load() & 0x7
    }

    // Store a value in the register in either the X or Y latched value
    pub fn store(&mut self, value: u8) {
        match self.latch {
            false => self.x.store(value),
            true => self.y.store(value),
        }

        self.latch = !self.latch;
    }

    pub fn reset_latch(&mut self) {
        self.latch = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write() {
        let mut scroll = ScrollRegister::new();
        assert_eq!(scroll.x.load(), 0);
        assert_eq!(scroll.y.load(), 0);

        scroll.store(0xFF);
        scroll.store(0x11);

        assert_eq!(scroll.x.load(), 0xFF);
        assert_eq!(scroll.y.load(), 0x11);
    }

    #[test]
    fn fine_x_scroll() {
        let mut scroll = ScrollRegister::new();
        scroll.store(0xFE);

        assert_eq!(0x6, scroll.fine_x_scroll());
    }
}

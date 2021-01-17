pub struct ScrollRegister {
    pub x: u8,
    pub y: u8,
    latch: bool,
}

impl ScrollRegister {
    pub fn new() -> Self {
        ScrollRegister {
            x: 0,
            y: 0,
            latch: false,
        }
    }

    pub fn write(&mut self, data: u8) {
        match self.latch {
            false => self.x = data,
            true => self.y = data,
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
        assert_eq!(scroll.x, 0);
        assert_eq!(scroll.y, 0);

        scroll.write(0xFF);
        scroll.write(0x11);

        assert_eq!(scroll.x, 0xFF);
        assert_eq!(scroll.y, 0x11);
    }
}

pub struct AddressRegister {
    value: u16,
    high_pointer: bool,
}

impl AddressRegister {
    pub fn new() -> Self {
        AddressRegister {
            value: 0,
            high_pointer: true,
        }
    }

    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn write(&mut self, data: u8) {
        match self.high_pointer {
            true => self.value = self.value & 0x00FF | (data as u16) << 8,
            false => self.value = self.value & 0xFF00 | (data as u16),
        }

        self.high_pointer = !self.high_pointer;
    }

    pub fn increment(&mut self, inc: u8) {
        self.value = (self.value + (inc as u16)) % 0x4000;
    }

    pub fn reset_latch(&mut self) {
        self.high_pointer = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get() {
        let register = AddressRegister::new();

        assert_eq!(register.get(), 0);
    }

    #[test]
    fn test_update() {
        let mut register = AddressRegister::new();

        register.write(0x12);
        register.write(0x34);

        assert_eq!(register.get(), 0x1234);
    }

    #[test]
    fn test_increment() {
        let mut register = AddressRegister::new();
        register.write(0x12);
        register.write(0x34);

        register.increment(1);
        assert_eq!(register.get(), 0x1235);

        register.write(0x3F);
        register.write(0xFF);

        register.increment(2);
        assert_eq!(register.get(), 0x0001);
    }

    #[test]
    fn test_reset_latch() {
        let mut register = AddressRegister::new();
        register.write(0x12);
        register.reset_latch();
        register.write(0x34);
        register.write(0x56);

        assert_eq!(register.get(), 0x3456);
    }
}

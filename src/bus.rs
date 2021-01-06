pub struct Bus {
    ram: [u8; 0x10000],
}

impl Bus {
    pub fn new() -> Bus {
        Bus { ram: [0; 0x10000] }
    }

    /// Reads a byte from the interface at the given address
    pub fn read(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    /// Writes a byte to the interface at the given address
    pub fn write(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reads_and_writes_to_ram() {
        let mut bus = Bus::new();

        bus.write(0x1234, 0xff);
        let result = bus.read(0x1234);

        assert_eq!(result, 0xff);
    }
}

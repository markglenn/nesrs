const CARTRIDGE_SIZE: usize = 0xBFE0;
pub struct Bus {
    internal_ram: [u8; 0x800],
    cartridge: [u8; CARTRIDGE_SIZE],
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
            internal_ram: [0; 0x800],
            cartridge: [0; CARTRIDGE_SIZE],
        }
    }

    /// Reads a byte from the interface at the given address
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0..=0x1FFF => self.internal_ram[(address & 0x7FF) as usize],
            0x2000..=0x3FFF => panic!("NES PPU registers"), /* Mirror every 8 bytes */
            0x4000..=0x4017 => panic!("NES APU and I/O registers"),
            0x4018..=0x401F => panic!("APU and I/O functionality that is normally disabled."),
            0x4020..=0xFFFF => self.cartridge[(address - 0x4020) as usize],
        }
    }

    /// Writes a byte to the interface at the given address
    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0..=0x1FFF => self.internal_ram[(address & 0x7FF) as usize] = data,
            0x2000..=0x3FFF => panic!("NES PPU registers"), /* Mirror every 8 bytes */
            0x4000..=0x4017 => panic!("NES APU and I/O registers"),
            0x4018..=0x401F => panic!("APU and I/O functionality that is normally disabled."),
            0x4020..=0xFFFF => self.cartridge[(address - 0x4020) as usize] = data,
        };
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

    #[test]
    fn reads_and_writes_mirrors_in_ram() {
        let mut bus = Bus::new();

        bus.write(0x0000, 0xff);
        assert_eq!(bus.read(0x0800), 0xff);
        assert_eq!(bus.read(0x1000), 0xff);
        assert_eq!(bus.read(0x1800), 0xff);
    }
}

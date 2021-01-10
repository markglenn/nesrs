use crate::{cartridge::rom::NESRom, ppu::Ppu};

const RAM_SIZE: usize = 0x800;
pub struct Bus {
    internal_ram: [u8; RAM_SIZE],
    cartridge: Box<NESRom>,
    ppu: Ppu,
}

impl Bus {
    pub fn new(rom: Box<NESRom>) -> Bus {
        Bus {
            internal_ram: [0; RAM_SIZE],
            cartridge: rom,
            ppu: Ppu::new(),
        }
    }

    /// Reads a byte from the interface at the given address
    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0..=0x1FFF => self.internal_ram[(address & 0x7FF) as usize],
            0x2000..=0x3FFF => self.ppu.read(address & 0x7),
            0x4000..=0x4017 => panic!("Reading from NES APU and I/O registers"),
            0x4018..=0x401F => {
                panic!("Reading from APU and I/O functionality that is normally disabled.")
            }
            0x4020..=0xFFFF => self.cartridge.read(address),
        }
    }

    pub fn read_word(&mut self, address: u16) -> u16 {
        let low_byte = self.read(address) as u16;
        let high_byte = self.read(address + 1) as u16;

        low_byte | (high_byte << 8)
    }

    /// Writes a byte to the interface at the given address
    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0..=0x1FFF => self.internal_ram[(address & 0x7FF) as usize] = data,
            0x2000..=0x3FFF => panic!("NES PPU registers"), /* Mirror every 8 bytes */
            0x4000..=0x4017 => panic!("NES APU and I/O registers"),
            0x4018..=0x401F => panic!("APU and I/O functionality that is normally disabled."),
            0x4020..=0xFFFF => self.cartridge.write(address, data),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reads_and_writes_to_ram() {
        let cartridge = Box::new(NESRom::from_file("./priv/nestest.nes").unwrap());
        let mut bus = Bus::new(cartridge);

        bus.write(0x1234, 0xff);
        let result = bus.read(0x1234);

        assert_eq!(result, 0xff);
    }

    #[test]
    fn reads_and_writes_mirrors_in_ram() {
        let cartridge = Box::new(NESRom::from_file("./priv/nestest.nes").unwrap());
        let mut bus = Bus::new(cartridge);

        bus.write(0x0000, 0xff);
        assert_eq!(bus.read(0x0800), 0xff);
        assert_eq!(bus.read(0x1000), 0xff);
        assert_eq!(bus.read(0x1800), 0xff);
    }
}

use crate::{apu::Apu, cartridge::rom::NESRom, ppu::Ppu};

const RAM_SIZE: usize = 0x800;
pub struct Bus {
    internal_ram: [u8; RAM_SIZE],
    cartridge: Box<NESRom>,
    ppu: Ppu,
    apu: Apu,
    pub cyc: u64,
}

impl Bus {
    pub fn new(rom: Box<NESRom>) -> Bus {
        Bus {
            internal_ram: [0; RAM_SIZE],
            cartridge: rom,
            ppu: Ppu::new(),
            apu: Apu::new(),
            cyc: 0,
        }
    }

    /// Reads a byte from the interface at the given address
    pub fn read(&mut self, address: u16) -> u8 {
        self.tick();
        self.unclocked_read(address)
    }

    pub fn unclocked_read(&mut self, address: u16) -> u8 {
        match address {
            0..=0x1FFF => self.internal_ram[(address & 0x7FF) as usize],
            0x2000..=0x3FFF => self.ppu.read(address & 0x7),
            0x4000..=0x4017 => self.apu.read(address),
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

    pub fn unclocked_read_word(&mut self, address: u16) -> u16 {
        let low_byte = self.unclocked_read(address) as u16;
        let high_byte = self.unclocked_read(address + 1) as u16;

        low_byte | (high_byte << 8)
    }

    /// Writes a byte to the interface at the given address
    pub fn write(&mut self, address: u16, data: u8) {
        self.tick();
        match address {
            0..=0x1FFF => self.internal_ram[(address & 0x7FF) as usize] = data,
            0x2000..=0x3FFF => panic!("NES PPU registers"), /* Mirror every 8 bytes */
            0x4000..=0x4017 => self.apu.write(address, data),
            0x4018..=0x401F => panic!("APU and I/O functionality that is normally disabled."),
            0x4020..=0xFFFF => self.cartridge.write(address, data),
        };
    }

    pub fn tick(&mut self) {
        self.cyc = self.cyc + 1;
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

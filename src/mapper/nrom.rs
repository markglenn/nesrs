use crate::cartridge::header::Header;

use super::Mapper;

const RAM_PAGE_SIZE: u16 = 8192;

#[derive(Debug, PartialEq)]
enum NRomType {
    NRom128 = 1,
    NRom256 = 2,
}

pub struct NRomMapper {
    nrom_type: NRomType,
    ram_size: u16,
}

impl NRomMapper {
    pub fn new(header: &Header) -> Self {
        let nrom_type = match header.prg_rom_pages {
            1 => NRomType::NRom128,
            2 => NRomType::NRom256,
            _ => panic!("Invalid NRom type {}", header.prg_rom_pages),
        };

        let ram_size = header.prg_ram_size() as u16;

        NRomMapper {
            nrom_type,
            ram_size,
        }
    }
}

impl Mapper for NRomMapper {
    fn map(&self, address: u16) -> u16 {
        // NROM-128 has duplicate roms across 0x8000 and 0xC000
        if self.nrom_type == NRomType::NRom128 && address >= 0xC000 {
            address - 0xC000 + self.ram_size
        } else if address >= 0x8000 {
            address - 0x8000 + self.ram_size
        } else {
            (address - 0x6000) % self.ram_size
        }
    }

    fn mirroring(&self) -> super::Mirroring {
        todo!()
    }
}

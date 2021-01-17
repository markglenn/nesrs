use crate::cartridge::header::Header;

use super::Mapper;

#[derive(Debug, PartialEq)]
enum NRomType {
    NRom128 = 1,
    NRom256 = 2,
}

pub struct NRomMapper {
    nrom_type: NRomType,
}

impl NRomMapper {
    pub fn new(header: &Header) -> Self {
        let nrom_type = match header.prg_rom_pages {
            1 => NRomType::NRom128,
            2 => NRomType::NRom256,
            _ => panic!("Invalid NRom type {}", header.prg_rom_pages),
        };

        NRomMapper { nrom_type }
    }
}

impl Mapper for NRomMapper {
    fn map(&self, address: u16) -> u16 {
        // NROM-128 has duplicate roms across 0x8000 and 0xC000
        if self.nrom_type == NRomType::NRom128 && address >= 0xC000 {
            address - 0xC000
        } else {
            address - 0x8000
        }
    }

    fn mirroring(&self) -> super::Mirroring {
        todo!()
    }
}

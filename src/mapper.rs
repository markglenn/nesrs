use crate::cartridge::header::Header;

use self::nrom::NRomMapper;

pub(crate) mod nrom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
}

pub trait Mapper {
    fn map(&self, address: u16) -> u16;
    fn mirroring(&self) -> Mirroring;
}

pub struct MapperFactory;

impl MapperFactory {
    pub fn create(header: &Header) -> Box<dyn Mapper> {
        match header.mapper {
            0 => Box::new(NRomMapper::new(header)),
            _ => panic!("Unsupported mapper {}", header.mapper),
        }
    }
}

use std::{
    fs::File,
    io::{self, SeekFrom},
};

use super::header::Header;
use crate::mapper::{Mapper, MapperFactory};

pub struct NESRom {
    pub header: Header,
    data: Vec<u8>,
    mapper: Box<dyn Mapper>,
}

impl NESRom {
    pub fn new<S: io::Read + io::Seek>(stream: &mut S) -> io::Result<NESRom> {
        let header = Header::new(stream)?;

        // Skip over the trainer data for now
        if header.has_trainer {
            stream.seek(SeekFrom::Current(512))?;
        }

        let mapper = MapperFactory::create(&header);
        let mut data = vec![0; header.prg_rom_pages * 0x4000 + header.chr_rom_pages * 0x2000];

        stream.read_exact(&mut data)?;

        Ok(NESRom {
            header,
            mapper,
            data,
        })
    }

    pub fn from_file(filename: &str) -> io::Result<NESRom> {
        let mut f = File::open(filename)?;

        NESRom::new(&mut f)
    }

    pub fn read(self: &Self, address: u16) -> u8 {
        let internal_address = self.mapper.map(address);
        self.data[internal_address as usize]
    }

    pub fn write(self: &mut Self, address: u16, data: u8) {
        let internal_address = self.mapper.map(address);
        self.data[internal_address as usize] = data;
    }
}

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

        let length = 0x2000 + header.prg_rom_pages * 0x4000 + header.chr_rom_pages * 0x2000;

        let mapper = MapperFactory::create(&header);
        let mut data = vec![0; length];

        stream.read_exact(&mut data[0x2000..length])?;

        Ok(NESRom {
            header,
            mapper,
            data,
        })
    }

    pub fn chr_rom(&self) -> Vec<u8> {
        if self.header.chr_rom_pages > 0 {
            let start = self.header.prg_rom_pages * 0x4000 + 0x2000;
            let length = self.header.chr_rom_pages * 0x2000;

            self.data[start..start + length].to_vec()
        } else {
            vec![0; self.header.chr_ram_pages * 0x2000]
        }
    }

    pub fn from_file(filename: &str) -> io::Result<NESRom> {
        let mut f = File::open(filename)?;

        NESRom::new(&mut f)
    }

    pub fn read(self: &Self, address: u16) -> u8 {
        let internal_address = self.mapper.map(address);

        if internal_address < 0x2000 && self.header.chr_ram_pages == 0 {
            0
        } else {
            self.data[internal_address as usize]
        }
    }

    pub fn write(self: &mut Self, address: u16, data: u8) {
        let internal_address = self.mapper.map(address);
        self.data[internal_address as usize] = data;
    }
}

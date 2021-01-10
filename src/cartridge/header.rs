use std::cmp;
use std::io::{self, ErrorKind, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::mapper::Mirroring;

// NES<EOF>
const MAGIC_HEADER: u32 = 0x1A53454E;

const FLAG6: usize = 0;
const FLAG7: usize = 1;
const FLAG8: usize = 2;
const _FLAG9: usize = 3;
const _FLAG10: usize = 4;

#[derive(Debug)]
pub struct Header {
    pub prg_rom_pages: usize,
    pub prg_ram_pages: usize,
    pub chr_rom_pages: usize,
    pub chr_ram_pages: usize,
    pub mapper: u8,
    pub has_trainer: bool,
    pub mirroring: Mirroring,
}

impl Header {
    pub fn new<S: io::Read + io::Seek>(stream: &mut S) -> io::Result<Header> {
        if stream.read_u32::<LittleEndian>()? != MAGIC_HEADER {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Invalid file format",
            ));
        }

        let prg_rom_pages = stream.read_u8()? as usize;
        let chr_rom_pages = stream.read_u8()? as usize;
        let chr_ram_pages = if chr_rom_pages == 0 { 1 } else { 0 };

        let mut flags = [0; 5];
        stream.read_exact(&mut flags)?;

        let mirroring = if flags[FLAG6] & 1 == 0 {
            Mirroring::Horizontal
        } else {
            Mirroring::Vertical
        };

        let prg_ram_pages = usize::from(cmp::max(1, flags[FLAG8]));
        let has_trainer = flags[0] & 0b100 > 0;
        let mapper = (flags[FLAG7] >> 4) | (flags[FLAG8] & 0xF0);

        // Unused part of header
        stream.seek(SeekFrom::Current(5))?;

        return Ok(Header {
            prg_rom_pages,
            prg_ram_pages,
            chr_rom_pages,
            chr_ram_pages,
            mapper,
            has_trainer,
            mirroring,
        });
    }
}

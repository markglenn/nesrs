use byteorder::{LittleEndian, ReadBytesExt};
use io::ErrorKind;
use std::{fs::File, io};

// NES<EOF>
const MAGIC_HEADER: u32 = 0x1A53454E;

#[derive(Debug)]
pub struct NESRom {
    prg_rom: Vec<u8>,
    chr_rom: Option<Vec<u8>>,
    trainer: Option<Vec<u8>>,
}

impl NESRom {
    pub fn new<S: io::Read + io::Seek>(stream: &mut S) -> io::Result<NESRom> {
        let magic = stream.read_u32::<LittleEndian>()?;

        if magic != MAGIC_HEADER {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "Invalid file format",
            ));
        }

        let mut prg_rom = vec![0; (stream.read_u8()? as usize) * 0x4000];
        let chr_rom_size = stream.read_u8()? as usize;

        let mut flags = vec![0; 10];
        stream.read_exact(&mut flags)?;

        let trainer = if flags[0] & 0b1000 > 0 {
            let mut buf = vec![0; 0x200];
            stream.read_exact(&mut buf)?;
            Some(buf)
        } else {
            None
        };

        let _mapper = (flags[1] >> 4) | (flags[2] & 0xF0);
        stream.read_exact(&mut prg_rom)?;

        let chr_rom = if chr_rom_size > 0 {
            let mut buf = vec![0; chr_rom_size * 0x2000];
            stream.read_exact(&mut buf[..])?;
            Some(buf)
        } else {
            None
        };

        Ok(NESRom {
            prg_rom,
            chr_rom,
            trainer,
        })
    }

    pub fn from_file(filename: &str) -> io::Result<NESRom> {
        let mut f = File::open(filename)?;

        NESRom::new(&mut f)
    }
}

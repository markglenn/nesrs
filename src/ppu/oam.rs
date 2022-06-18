pub struct OAM {
    // Buffer for the OAM
    buffer: Vec<u8>,

    // Current write address
    address: usize,

    // Size in bytes of the OAM
    size: usize,
}

pub struct Tile<'a> {
    buffer: &'a [u8],
}

impl Tile<'_> {
    pub fn y_pos(&self) -> usize {
        self.buffer[0] as usize
    }

    pub fn tile_idx(&self) -> usize {
        self.buffer[1] as usize
    }

    pub fn attributes(&self) -> u8 {
        self.buffer[2]
    }

    pub fn x_pos(&self) -> usize {
        self.buffer[3] as usize
    }

    pub fn flip_horizontal(&self) -> bool {
        self.buffer[2] & 0b1000_0000 != 0
    }

    pub fn flip_vertical(&self) -> bool {
        self.buffer[2] & 0b0100_0000 != 0
    }

    pub fn palette_idx(&self) -> u8 {
        self.buffer[2] & 0b0000_0011
    }
}

impl OAM {
    pub fn new(size: usize) -> OAM {
        OAM {
            buffer: vec![0; size * 4],
            address: 0,
            size: size * 4,
        }
    }

    pub fn store_address(&mut self, address: u8) {
        self.address = address as usize;
    }

    pub fn store(&mut self, value: u8) {
        self.buffer[self.address as usize] = value;
        self.address = (self.address + 1) % self.size;
    }

    pub fn load(&self) -> u8 {
        self.buffer[self.address as usize]
    }

    pub fn tile(&self, idx: usize) -> Tile {
        Tile {
            buffer: &self.buffer[idx * 4..idx * 4 + 3],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn store_address() {
        let mut oam = OAM::new(8);

        assert_eq!(0, oam.address);
        oam.store_address(0x7);
        assert_eq!(7, oam.address);
    }

    #[test]
    fn store() {
        let mut oam = OAM::new(8);

        for i in 1..=8 {
            oam.store(i);
        }

        assert_eq!(0, oam.address);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8].to_vec(), oam.buffer);
    }
}

pub struct OAM {
    // Buffer for the OAM
    buffer: Vec<u8>,

    // Current write address
    address: usize,

    // Size in bytes of the OAM
    size: usize,
}

pub struct Tile<'a> {
    // Stores an immutable slice from the OAM
    buffer: &'a [u8],
}

use bitfield::bitfield;

bitfield! {
    pub struct SpriteAttributes(u8);

    impl Debug;

    pub palette, _: 1, 0;
    pub priority, _: 5;
    pub flip_horizontal, _: 6;
    pub flip_vertical, _: 7;
}

impl Tile<'_> {
    // Y position on the screen
    pub fn y_pos(&self) -> usize {
        self.buffer[0] as usize
    }

    // Tile index into CHR ROM
    pub fn tile_idx(&self) -> usize {
        self.buffer[1] as usize
    }

    // Raw attribute data
    pub fn attributes(&self) -> SpriteAttributes {
        SpriteAttributes(self.buffer[2] & 0xE3)
    }

    // X position on the screen
    pub fn x_pos(&self) -> u8 {
        self.buffer[3]
    }

    // Flip this sprite horizontally?
    pub fn flip_horizontal(&self) -> bool {
        self.buffer[2] & 0b0100_0000 != 0
    }

    // Flip this sprite vertically?
    pub fn flip_vertical(&self) -> bool {
        self.buffer[2] & 0b1000_0000 != 0
    }

    // Index into the sprite palette
    pub fn palette_idx(&self) -> u8 {
        self.buffer[2] & 0b0000_0011
    }

    pub fn priority(&self) -> bool {
        self.buffer[2] & 0b0010_0000 != 0
    }

    pub fn visible_on_scanline(&self, scanline: usize, sprite_height: usize) -> bool {
        let y_pos = self.y_pos();

        y_pos >= scanline && y_pos < scanline + sprite_height
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

    // Store the address into the OAM
    pub fn store_address(&mut self, address: u8) {
        self.address = address as usize;
    }

    // Store a value in the OAM and increment the address
    pub fn store(&mut self, value: u8) {
        self.buffer[self.address] = value;
        self.address = (self.address + 1) % self.size;
    }

    pub fn store_tile(&mut self, tile: Tile) {
        let buffer = tile.buffer;

        self.store(buffer[0]);
        self.store(buffer[1]);
        self.store(buffer[2]);
        self.store(buffer[3]);
    }

    // Load a value from the OAM at the current address
    pub fn load(&self) -> u8 {
        self.buffer[self.address]
    }

    // Tile helper value for rendering
    pub fn tile(&self, idx: usize) -> Tile {
        let offset = idx * 4;
        Tile {
            buffer: &self.buffer[offset..=offset + 3],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.address = 0;
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
        let mut oam = OAM::new(2);

        for i in 1..=8 {
            oam.store(i);
        }

        assert_eq!(0, oam.address);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8].to_vec(), oam.buffer);
    }

    #[test]
    fn tile() {
        let mut oam = OAM::new(1);
        oam.store_address(2);
        oam.store(0b10101011);

        let tile = oam.tile(0);

        assert_eq!(false, tile.flip_horizontal());
        assert_eq!(true, tile.flip_vertical());
        assert_eq!(true, tile.priority());
        assert_eq!(3, tile.palette_idx());

        assert_eq!(0b10100011, tile.attributes());
    }
}

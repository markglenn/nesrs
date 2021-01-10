use bitfield::bitfield;

// Status(0x2002)
bitfield! {
    pub struct Status(u8);

    impl Debug;

    pub sprite_overflow, set_sprite_overflow: 5;
    pub sprite_0_hit, set_sprite_0_hit: 6;
    pub vblank, set_vblank: 7;
    pub read, _: 7, 0;
}
pub struct Ppu {
    status: Status,
    latch: bool,
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            status: Status(0),
            latch: false,
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x2 => self.read_status(),
            _ => panic!("Invalid PPU Address: 0x{:04x}", address),
        }
    }

    fn read_status(&mut self) -> u8 {
        let result = self.status.read();

        self.status.set_vblank(false);
        self.latch = false;
        result
    }
}

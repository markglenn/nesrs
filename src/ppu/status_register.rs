use bitfield::bitfield;

bitfield! {
    pub struct StatusRegister(u8);

    impl Debug;

    pub sprite_overflow, set_sprite_overflow: 5;
    pub sprite_0_hit, set_sprite_0_hit: 6;
    pub vblank, set_vblank: 7;
    pub get, _: 7, 0;
}

impl StatusRegister {
    pub fn new() -> Self {
        StatusRegister(0b1000_0000)
    }

    pub fn read(&mut self) -> u8 {
        let data = self.get();
        self.set_vblank(false);

        data
    }
}

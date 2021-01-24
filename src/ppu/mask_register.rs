use bitfield::bitfield;

bitfield! {
    pub struct MaskRegister(u8);
    impl Debug;
    /*

        7  bit  0
        ---- ----
        BGRs bMmG
        |||| ||||
        |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
        |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
        |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
        |||| +---- 1: Show background
        |||+------ 1: Show sprites
        ||+------- Emphasize red (green on PAL/Dendy)
        |+-------- Emphasize green (red on PAL/Dendy)
        +--------- Emphasize blue

    */
    pub grayscale, _: 0;
    pub leftmost_8_background, _: 1;
    pub leftmost_8_sprite, _: 2;
    pub show_background, _: 3;
    pub show_sprites, _: 4;
    pub emphasize_red, _: 5;
    pub emphasize_green, _: 6;
    pub emphasize_blue, _: 7;

    pub get, set: 7, 0;
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister(0)
    }

    pub fn write(&mut self, data: u8) {
        self.set(data);
    }

    pub fn show_background_at(&self, x: usize) -> bool {
        self.show_background() && (self.leftmost_8_background() || x > 8)
    }
}

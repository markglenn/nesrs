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

    pub load, store: 7, 0;
}

impl MaskRegister {
    pub fn new() -> Self {
        MaskRegister(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn grayscale() {
        let mut mask = MaskRegister::new();
        assert_eq!(false, mask.grayscale());
        mask.store(0b0000_0001);
        assert_eq!(true, mask.grayscale());
    }

    #[test]
    fn leftmost_8_background() {
        let mut mask = MaskRegister::new();
        assert_eq!(false, mask.leftmost_8_background());
        mask.store(0b0000_0010);
        assert_eq!(true, mask.leftmost_8_background());
    }

    #[test]
    fn leftmost_8_sprite() {
        let mut mask = MaskRegister::new();
        assert_eq!(false, mask.leftmost_8_sprite());
        mask.store(0b0000_0100);
        assert_eq!(true, mask.leftmost_8_sprite());
    }

    #[test]
    fn show_background() {
        let mut mask = MaskRegister::new();
        assert_eq!(false, mask.show_background());
        mask.store(0b0000_1000);
        assert_eq!(true, mask.show_background());
    }

    #[test]
    fn show_sprites() {
        let mut mask = MaskRegister::new();
        assert_eq!(false, mask.show_sprites());
        mask.store(0b0001_0000);
        assert_eq!(true, mask.show_sprites());
    }

    #[test]
    fn emphasize_red() {
        let mut mask = MaskRegister::new();
        assert_eq!(false, mask.emphasize_red());
        mask.store(0b0010_0000);
        assert_eq!(true, mask.emphasize_red());
    }

    #[test]
    fn emphasize_green() {
        let mut mask = MaskRegister::new();
        assert_eq!(false, mask.emphasize_green());
        mask.store(0b0100_0000);
        assert_eq!(true, mask.emphasize_green());
    }

    #[test]
    fn emphasize_blue() {
        let mut mask = MaskRegister::new();
        assert_eq!(false, mask.emphasize_blue());
        mask.store(0b1010_0000);
        assert_eq!(true, mask.emphasize_blue());
    }
}

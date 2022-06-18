use bitfield::bitfield;

bitfield! {
    pub struct ControlRegister(u8);

    impl Debug;

    pub nametable, _: 1, 0;
    pub vram_addr_inc, _: 2;
    sprite_pattern_addr, _: 3;
    background_pattern_addr, _: 4;
    pub sprite_size, _: 5;
    pub master_slave_select, _: 6;
    pub generate_nmi, _: 7;
    pub load, store: 7, 0;
}

impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister(0)
    }

    pub fn vram_address_increment(&self) -> u8 {
        if self.vram_addr_inc() {
            32
        } else {
            1
        }
    }

    pub fn base_nametable_address(&self) -> u16 {
        match self.nametable() {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => unreachable!(),
        }
    }

    pub fn background_pattern_address(&self) -> usize {
        match self.background_pattern_addr() {
            false => 0x0000,
            true => 0x1000,
        }
    }

    pub fn sprite_pattern_address(&self) -> usize {
        match self.sprite_pattern_addr() {
            false => 0x0000,
            true => 0x1000,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_sets_value() {
        let mut ctrl = ControlRegister::new();
        assert_eq!(ctrl.load(), 0);

        ctrl.store(0b101);

        assert_eq!(ctrl.vram_address_increment(), 32);
        assert_eq!(ctrl.base_nametable_address(), 0x2400);
    }
}

use super::frame::Frame;
use super::registers::Registers;
use super::sprite::Sprite;
use super::PPUResult;

pub struct Renderer {
    pub cycle: usize,
    pub scanline: usize,
    pub frame: Frame,

    pub oam_data: [u8; 256],
    pub secondary_oam: Vec<Sprite>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            cycle: 0,
            scanline: 0,
            frame: Frame::new(),
            oam_data: [0; 256],
            secondary_oam: Vec::with_capacity(8),
        }
    }

    pub fn tick(&mut self, regs: &mut Registers) -> PPUResult {
        let result = match (self.scanline, self.cycle) {
            (0..=239, _) => {
                self.tick_sprites();
                PPUResult::None
            }
            (240, 0) => PPUResult::FrameComplete,
            (240, 1) => {
                regs.status.set_vblank(true);
                PPUResult::None
            }
            _ => PPUResult::None,
        };

        self.tick_cycle();

        result
    }

    fn tick_cycle(&mut self) {
        self.cycle += 1;

        if self.cycle >= 341 {
            self.cycle %= 341;
            self.scanline += 1;

            if self.scanline >= 262 {
                self.scanline = 0;
            }
        }
    }

    fn tick_sprites(&mut self) {
        match self.cycle {
            _ => (),
        }
    }

    // Find all sprites that are on this scanline
    fn load_scanline_sprites(&mut self, regs: &mut Registers) {
        self.secondary_oam.clear();
        let sprite_size: usize = if regs.ctrl.sprite_size() { 16 } else { 8 };

        for addr in (0..256).step_by(4) {
            let sprite = Sprite::new(&self.oam_data[addr..addr + 4]);

            let top = sprite.y as usize;
            let bottom = top + sprite_size;

            // Is this sprite visible on this scanline
            if self.scanline >= top && self.scanline < bottom {
                // We hit our maximum.  Set the overflow bit and move on
                if self.secondary_oam.len() == 8 {
                    regs.status.set_sprite_overflow(true);
                    break;
                }

                // Add this sprite to the current scanline
                self.secondary_oam.push(sprite);
            }
        }
    }
}

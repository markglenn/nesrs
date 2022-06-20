pub mod frame;
pub mod palette;

mod address_register;
mod control_register;
mod mask_register;
mod oam;
mod scroll_register;
mod status_register;

use std::fmt::Debug;

use address_register::AddressRegister;
use control_register::ControlRegister;
use mask_register::MaskRegister;
use scroll_register::ScrollRegister;
use status_register::StatusRegister;

use crate::hardware::interrupt::Interrupt;
use crate::hardware::register::Register;
use crate::mapper::Mirroring;

use oam::OAM;

use self::frame::Frame;
use self::oam::SpriteAttributes;

pub struct Ppu {
    pub palette_table: [u8; 0x20],
    pub vram: [u8; 0x800],
    pub chr_rom: Vec<u8>,
    internal: u8,         // Internal bus data buffer
    mirroring: Mirroring, // Mirroring mode

    pub ctrl: ControlRegister,  // 0x2000
    mask: MaskRegister,         // 0x2001
    status: StatusRegister,     // 0x2002
    addr: AddressRegister,      // 0x2006
    pub scroll: ScrollRegister, // 0x2005

    // Contains the 64 sprites for the current frame
    pub primary_oam: OAM,

    // Contains the 8 sprites for the current scanline
    secondary_oam: OAM,

    // Timing information for the PPU
    cycle: usize,
    scanline: usize,

    // Internal background registers
    background_patterns: [Register<u16>; 2],
    background_palette: [Register<u8>; 2],

    // Internal sprite registers
    sprite_patterns_low: [Register<u8>; 8],
    sprite_patterns_high: [Register<u8>; 8],
    sprite_attributes: [SpriteAttributes; 8],
    sprite_x_positions: [Register<u8>; 8],

    pub frame: Frame,
}

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Ppu {
        Ppu {
            internal: 0,
            palette_table: [0; 32],
            vram: [0; 2048],
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            addr: AddressRegister::new(),
            scroll: ScrollRegister::new(),

            mirroring,
            cycle: 0,

            primary_oam: OAM::new(64),
            secondary_oam: OAM::new(8),

            scanline: 241,
            chr_rom,

            background_patterns: [Register::<u16>::new(); 2],
            background_palette: [Register::<u8>::new(); 2],

            sprite_patterns_low: [Register::<u8>::new(); 8],
            sprite_patterns_high: [Register::<u8>::new(); 8],
            sprite_attributes: [
                SpriteAttributes(0),
                SpriteAttributes(0),
                SpriteAttributes(0),
                SpriteAttributes(0),
                SpriteAttributes(0),
                SpriteAttributes(0),
                SpriteAttributes(0),
                SpriteAttributes(0),
            ],

            sprite_x_positions: [Register::<u8>::new(); 8],
            frame: Frame::new(),
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                // panic!(
                //     "Attempted to read from write only register at address: {:04X}",
                //     address
                // );
                0
            }
            0x2002 => self.status.load(),
            0x2004 => self.primary_oam.load(),
            0x2007 => self.read_data(),
            _ => self.read(address & 0x2007),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x2000 => self.ctrl.store(value),
            0x2001 => self.mask.store(value),
            0x2002 => panic!("Attempted to write to PPU status register"),
            0x2003 => self.primary_oam.store_address(value),
            0x2004 => self.primary_oam.store(value),
            0x2005 => self.scroll.store(value),
            0x2006 => self.addr.write(value),
            0x2007 => self.write_data(value),

            _ => self.write(address & 0x2007, value),
        }
    }

    pub fn read_data(&mut self) -> u8 {
        let address = self.addr.get();
        self.addr.increment(self.ctrl.vram_address_increment());

        match address {
            0..=0x1FFF => {
                let result = self.internal;
                self.internal = self.chr_rom[address as usize];
                result
            }
            0x2000..=0x2FFF => {
                let result = self.internal;
                self.internal = self.vram[self.mirror_vram_addr(address) as usize];
                result
            }
            0x3F00..=0x3FFF => {
                // Side effect: When reading from palette, it also reads from VRAM into buffer
                self.internal = self.vram[self.mirror_vram_addr(address) as usize];
                self.palette_table[self.mirror_palette(address)]
            }
            _ => unimplemented!("Attempted to read from #{:04X}", address),
        }
    }

    pub fn write_data(&mut self, data: u8) {
        let address = self.addr.get();
        self.addr.increment(self.ctrl.vram_address_increment());

        match address {
            0..=0x1FFF => self.chr_rom[address as usize] = data,
            0x2000..=0x3EFF => self.vram[self.mirror_vram_addr(address) as usize] = data,
            0x3F00..=0x3FFF => self.palette_table[self.mirror_palette(address)] = data,
            _ => unimplemented!("Attempted to write to #{:04X}", address),
        }
    }

    pub fn tick(&mut self, nmi: &mut Interrupt) {
        self.cycle += 1;

        if self.cycle >= 1 && self.cycle <= 256 {
            self.render_sprites();
        }

        if self.cycle == 341 {
            self.cycle = 0;
            self.scanline += 1;

            match self.scanline {
                // Faked for Super Mario Bros
                30 => self.status.set_sprite_0_hit(true),
                241 => self.start_vblank(nmi),
                261 => {
                    self.load_shift_registers();
                    // No longer in vblank
                    self.status.set_vblank(false);
                    self.scanline = 0;
                    self.status.set_sprite_0_hit(false);
                }
                _ => (),
            }
        }
    }

    fn start_vblank(&mut self, nmi: &mut Interrupt) {
        self.status.set_vblank(true);

        if self.ctrl.generate_nmi() {
            nmi.schedule(1);
        }
    }

    pub fn write_oam_dma(&mut self, data: &[u8]) {
        for i in 0..256 {
            self.primary_oam.store(data[i]);
        }
    }

    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]

    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    fn mirror_vram_addr(&self, addr: u16) -> usize {
        let vram_index = (addr & 0x0FFF) as usize;
        let name_table = vram_index / 0x400;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) => vram_index - 0x800,
            (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    fn mirror_palette(&self, addr: u16) -> usize {
        (match addr {
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => addr & 0x0F,
            _ => addr & 0x1F,
        }) as usize
    }

    fn load_shift_registers(&mut self) {
        let scanline = if self.scanline == 261 {
            0
        } else {
            self.scanline
        };

        // Load the 8 sprites for the line
        if self.cycle == 257 {
            self.load_scanline_sprites(scanline);
        }
    }

    fn load_scanline_sprites(&mut self, scanline: usize) {
        let mut sprite_count = 0;
        self.secondary_oam.clear();

        let sprite_height = self.ctrl.sprite_height();
        let bank = self.ctrl.sprite_pattern_address();

        // Loop through all the sprites available to the frame
        for i in 0..64 {
            // Pull the attribute tile for this sprite
            let tile = self.primary_oam.tile(i);

            // If it's visible, add it to our visible sprite list
            if tile.visible_on_scanline(scanline, sprite_height) {
                let offset = bank + i * 16;
                let pattern = &self.chr_rom[offset..=offset + 15];
                let y = scanline - tile.y_pos();

                // Load the 4 registers for this sprite
                self.sprite_patterns_low[sprite_count].store(pattern[y + 8]);
                self.sprite_patterns_high[sprite_count].store(pattern[y]);

                self.sprite_x_positions[sprite_count].store(tile.x_pos());
                self.sprite_attributes[sprite_count] = tile.attributes();

                self.secondary_oam.store_tile(tile);

                sprite_count += 1;

                // Hit our max sprite line count?
                if sprite_count == 8 {
                    break;
                }
            }
        }
        // Fill the remaining sprites with empty values
        for i in sprite_count..8 {
            self.sprite_patterns_low[i].store(0);
            self.sprite_patterns_high[i].store(0);
        }
    }

    fn render_sprites(&mut self) {
        // Decrement the counters
        for i in 0..8 {
            if self.sprite_x_positions[i].load() > 0 {
                self.sprite_x_positions[i].decrement();
            } else {
                self.sprite_patterns_high[i].shift(1);
                self.sprite_patterns_low[i].shift(1);
            }
        }

        let sprite_palette = self.sprite_palette(0);

        let mut pixel: u8 = 0;
        for i in 0..8 {
            if self.sprite_x_positions[i].load() == 0 {
                let upper = self.sprite_patterns_high[i].load_bit(0);
                let lower = self.sprite_patterns_low[i].load_bit(0);

                pixel = (upper << 1) + lower;
            }

            if pixel > 0 {
                break;
            }
        }

        let rgb = match pixel {
            0 => (0, 0, 0), // skip pixel
            1 => palette::SYSTEM_PALETTE[sprite_palette[1] as usize],
            2 => palette::SYSTEM_PALETTE[sprite_palette[2] as usize],
            3 => palette::SYSTEM_PALETTE[sprite_palette[3] as usize],
            _ => unimplemented!(),
        };

        self.frame.set_pixel(self.cycle, self.scanline, rgb);
    }

    fn sprite_palette(&self, pallete_idx: u8) -> [u8; 4] {
        let start = 0x11 + (pallete_idx * 4) as usize;
        [
            0,
            self.palette_table[start],
            self.palette_table[start + 1],
            self.palette_table[start + 2],
        ]
    }
}

impl Debug for Ppu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CYC:{:3} SL:{:3} ST:{:?}",
            self.cycle, self.scanline, self.status
        )
    }
}

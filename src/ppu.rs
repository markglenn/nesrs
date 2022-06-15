pub mod frame;
pub mod palette;

mod address_register;
mod control_register;
mod mask_register;
mod scroll_register;
mod status_register;

use std::fmt::Debug;

use address_register::AddressRegister;
use control_register::ControlRegister;
use mask_register::MaskRegister;
use scroll_register::ScrollRegister;
use status_register::StatusRegister;

use crate::hardware::interrupt::Interrupt;
use crate::mapper::Mirroring;

pub struct Ppu {
    pub palette_table: [u8; 0x20],
    pub vram: [u8; 0x800],
    oam_addr: u8,
    pub chr_rom: Vec<u8>,
    internal: u8,         // Internal bus data buffer
    mirroring: Mirroring, // Mirroring mode

    pub ctrl: ControlRegister, // 0x2000
    mask: MaskRegister,        // 0x2001
    status: StatusRegister,    // 0x2002
    addr: AddressRegister,     // 0x2006
    scroll: ScrollRegister,    // 0x2005

    register_first_store: bool,

    // Contains the 64 sprites for the current frame
    pub primary_oam: [u8; 256],

    // Contains the 8 sprites for the current scanline
    secondary_oam: [u8; 32],
    // Timing information for the PPU
    cycle: usize,
    scanline: usize,
}

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Ppu {
        Ppu {
            internal: 0,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_addr: 0,
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            addr: AddressRegister::new(),
            register_first_store: true,
            scroll: ScrollRegister::new(),

            mirroring,
            cycle: 0,

            primary_oam: [0; 256],
            secondary_oam: [0; 32],

            scanline: 241,
            chr_rom,
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
            0x2004 => self.primary_oam[self.oam_addr as usize],
            0x2007 => self.read_data(),
            _ => self.read(address & 0x2007),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x2000 => self.ctrl.store(value),
            0x2001 => self.mask.store(value),
            0x2002 => panic!("Attempted to write to PPU status register"),
            0x2003 => self.oam_addr = value,
            0x2004 => self.write_oam_data(value),
            0x2005 => self.scroll.store(value),
            0x2006 => self.addr.write(value),
            0x2007 => self.write_data(value),

            _ => self.write(address & 0x2007, value),
        }
    }

    fn write_oam_data(&mut self, value: u8) {
        self.primary_oam[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
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

        if self.cycle == 341 {
            self.cycle = 0;
            self.scanline += 1;

            match self.scanline {
                30 => self.status.set_sprite_0_hit(true),
                241 => self.start_vblank(nmi),
                261 => {
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
            self.write_oam_data(data[i]);
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

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
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    oam_addr: u8,
    pub chr_rom: Vec<u8>,
    internal: u8,         // Internal bus data buffer
    mirroring: Mirroring, // Mirroring mode

    pub ctrl: ControlRegister, // 0x2000
    pub mask: MaskRegister,    // 0x2001
    status: StatusRegister,    // 0x2002
    scroll: ScrollRegister,    // 0x2005
    addr: AddressRegister,     // 0x2006

    dot: usize,
    scanline: usize,
}

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Ppu {
        Ppu {
            internal: 0,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_addr: 0,
            oam_data: [0; 256],
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            addr: AddressRegister::new(),
            scroll: ScrollRegister::new(),
            mirroring,
            dot: 0,
            scanline: 241,
            chr_rom,
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!(
                    "Attempted to read from write only register at address: {:04X}",
                    address
                );
            }
            0x2002 => self.status.read(),
            0x2004 => self.oam_data[self.oam_addr as usize],
            0x2007 => self.read_data(),
            _ => self.read(address & 0x2007),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x2000 => self.ctrl.write(data),
            0x2001 => self.mask.write(data),
            0x2002 => panic!("Attempted to write to PPU status register"),
            0x2003 => self.oam_addr = data,
            0x2004 => self.write_oam_data(data),
            0x2005 => self.scroll.write(data),
            0x2006 => self.addr.write(data),
            0x2007 => self.write_data(data),

            _ => self.write(address & 0x2007, data),
        }
    }

    fn write_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn read_data(&mut self) -> u8 {
        let address = self.addr.get();
        self.addr.increment(self.ctrl.vram_address_increment());

        match address {
            0..=0x1FFF => self.chr_rom[address as usize],
            0x2000..=0x2FFF => {
                let result = self.internal;
                self.internal = self.vram[self.mirror_vram_addr(address) as usize];
                result
            }
            0x3F00..=0x3FFF => {
                // 0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C are mirrors
                let mask = if address & 0x11 == 0x10 { 0x0F } else { 0x1F };

                // Side effect: When reading from palette, it also reads from VRAM into buffer
                self.internal = self.vram[self.mirror_vram_addr(address) as usize];

                self.palette_table[(address & mask) as usize]
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
            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                self.palette_table[(address & 0x0F) as usize] = data
            }
            0x3F00..=0x3FFF => self.palette_table[(address & 0x1F) as usize] = data,
            _ => unimplemented!("Attempted to write to #{:04X}", address),
        }
    }

    pub fn tick(&mut self, nmi: &mut Interrupt) {
        self.dot += 1;

        if self.dot >= 341 {
            self.dot %= 341;
            self.scanline += 1;
            match self.scanline {
                241 => self.start_vblank(nmi),
                261 => {
                    // No longer in vblank
                    self.status.set_vblank(false);
                    self.scanline = 0;
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
}

impl Debug for Ppu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CYC:{:3} SL:{:3} ST:{:02X}",
            self.dot,
            self.scanline,
            self.status.get()
        )
    }
}

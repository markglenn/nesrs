pub mod frame;

mod address_register;
mod control_register;
mod mask_register;
mod registers;
mod renderer;
mod scroll_register;
mod sprite;
mod status_register;

use registers::Registers;
use renderer::Renderer;
use std::fmt::Debug;

use crate::hardware::interrupt::Interrupt;
use crate::mapper::Mirroring;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PPUResult {
    Nmi,
    FrameComplete,
    None,
}

pub struct Ppu {
    pub palette_table: [u8; 0x20],
    pub vram: [u8; 0x800],
    oam_addr: u8,
    pub chr_rom: Vec<u8>,
    internal: u8,         // Internal bus data buffer
    open_bus_value: u8,   // Returned when reading from an open bus
    mirroring: Mirroring, // Mirroring mode

    pub registers: Registers,
    pub renderer: Renderer,
}

impl Ppu {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Ppu {
        Ppu {
            internal: 0,
            palette_table: [0; 32],
            vram: [0; 2048],
            oam_addr: 0,
            registers: Registers::new(),
            mirroring,
            chr_rom,
            open_bus_value: 0,
            renderer: Renderer::new(),
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {
        match address {
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                // Reading from a write only register return the open bus value
                // https://wiki.nesdev.com/w/index.php/Open_bus_behavior
                self.open_bus_value
            }
            0x2002 => self.registers.status.read(),
            0x2004 => self.renderer.oam_data[self.oam_addr as usize],
            0x2007 => self.read_data(),
            _ => self.read(address & 0x2007),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        self.open_bus_value = data;

        match address {
            0x2000 => self.registers.ctrl.write(data),
            0x2001 => self.registers.mask.write(data),
            0x2002 => panic!("Attempted to write to PPU status register"),
            0x2003 => self.oam_addr = data,
            0x2004 => self.write_oam_data(data),
            0x2005 => self.registers.scroll.write(data),
            0x2006 => self.registers.addr.write(data),
            0x2007 => self.write_data(data),

            _ => self.write(address & 0x2007, data),
        }
    }

    fn write_oam_data(&mut self, value: u8) {
        self.renderer.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn read_data(&mut self) -> u8 {
        let address = self.registers.addr.get();
        self.registers
            .addr
            .increment(self.registers.ctrl.vram_address_increment());

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
        let address = self.registers.addr.get();
        self.registers
            .addr
            .increment(self.registers.ctrl.vram_address_increment());

        match address {
            0..=0x1FFF => self.chr_rom[address as usize] = data,
            0x2000..=0x3EFF => self.vram[self.mirror_vram_addr(address) as usize] = data,
            0x3F00..=0x3FFF => self.palette_table[self.mirror_palette(address)] = data,
            _ => unimplemented!("Attempted to write to #{:04X}", address),
        }
    }

    pub fn tick(&mut self, nmi: &mut Interrupt) {
        match self.renderer.tick(&mut self.registers) {
            PPUResult::Nmi => {
                if self.registers.ctrl.generate_nmi() {
                    nmi.schedule(1);
                }
            }
            PPUResult::FrameComplete => (),
            PPUResult::None => (),
        };
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
            "CYC:{:3} SL:{:3} ST:{:02X}",
            self.renderer.cycle,
            self.renderer.scanline,
            self.registers.status.get()
        )
    }
}

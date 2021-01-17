#![allow(dead_code)]
mod apu;
mod cartridge;
mod hardware;
mod mapper;
mod ppu;

use cartridge::rom::NESRom;
use hardware::cpu::Cpu;

fn main() {
    let cartridge = Box::new(NESRom::from_file("priv/nestest.nes").unwrap());
    let mut cpu = Cpu::new(cartridge);
    cpu.reset();

    let mut i = 0;
    loop {
        i = i + 1;
        print!("{:7}  ", i);
        cpu.execute_next_opcode();

        if i >= 8991 {
            panic!();
        }
    }
}

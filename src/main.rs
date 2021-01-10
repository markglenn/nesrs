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

    loop {
        let mut name: String = String::new();
        cpu.execute_next_opcode();
        println!("{:#?}", cpu);
    }
}

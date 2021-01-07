mod address_mode;
mod hardware;
mod rom;

use hardware::bus::Bus;
use hardware::cpu::Cpu;
use rom::NESRom;

fn main() {
    println!("Hello, world!");

    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    println!("{:#?}", cpu);

    cpu.push_stack(&mut bus, 0xFF);
    cpu.push_stack_16(&mut bus, 0xAAAA);

    let _cartridge = NESRom::from_file("priv/nestest.nes").unwrap();
    println!("#{:#?}", cpu.pop_stack_16(&bus));
    println!("#{:#?}", cpu.pop_stack(&bus));
}

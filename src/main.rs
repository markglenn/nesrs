mod address_mode;
mod bus;
mod cpu;
mod opcode;

use cpu::Cpu;

fn main() {
    println!("Hello, world!");

    let cpu = Cpu::new();

    println!("{:#?}", cpu);
}

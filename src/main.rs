mod address_mode;
mod bus;
mod cpu;

use cpu::Cpu;

fn main() {
    println!("Hello, world!");

    let cpu = Cpu::new();

    cpu.print_status();
}

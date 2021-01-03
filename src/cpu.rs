use super::bus::Bus;

#[repr(u8)]
#[derive(Clone, Copy)]
enum CpuStatus {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    Decimal = 3,
    Overflow = 6,
    Negative = 7,
}

pub struct Cpu {
    // 6502 CPU Registers
    a: u8,   // Accumulator
    x: u8,   // X register (index)
    y: u8,   // Y register (index)
    pc: u16, // Program counter
    s: u8,   // Stack pointer
    p: u8,   // Status register

    bus: Bus,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0,
            bus: Bus::new(),
        }
    }

    pub fn print_status(&self) {
        println!(
            "a: {}, x: {}, y: {}, pc: {}, p: {}",
            self.a, self.x, self.y, self.pc, self.p
        );

        print!("[C: {}, ", self.is_bit_set(CpuStatus::Carry));
        print!("Z: {}, ", self.is_bit_set(CpuStatus::Zero));
        print!("I: {}, ", self.is_bit_set(CpuStatus::InterruptDisable));
        print!("D: {}, ", self.is_bit_set(CpuStatus::Decimal));
        print!("O: {}, ", self.is_bit_set(CpuStatus::Overflow));
        println!("N: {}]", self.is_bit_set(CpuStatus::Negative));
    }

    fn is_bit_set(&self, bit: CpuStatus) -> u8 {
        if self.s & 1 << (bit as u8) == (bit as u8) {
            1
        } else {
            0
        }
    }
}

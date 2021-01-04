use super::bus::Bus;

const STACK_PAGE: u16 = 0x0100;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum CpuStatus {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    Decimal = 3,
    Overflow = 6,
    Negative = 7,
}

pub struct Cpu {
    // 6502 CPU Registers
    pub a: u8, // Accumulator
    pub x: u8, // X register (index)
    pub y: u8, // Y register (index)
    pc: u16,   // Program counter
    sp: u8,    // Stack pointer
    p: u8,     // Status register
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            p: 0,
        }
    }

    pub fn push_stack(&mut self, bus: &mut Bus, value: u8) {
        bus.write(STACK_PAGE + u16::from(self.sp), value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn push_stack_16(&mut self, bus: &mut Bus, value: u16) {
        self.push_stack(bus, (value >> 8) as u8);
        self.push_stack(bus, (value & 0xFF) as u8);
    }

    pub fn set_flag(&mut self, bit: CpuStatus, is_set: bool) {
        self.p = (self.p & !(1 << bit as u8)) | (u8::from(is_set) << bit as u8);
    }

    pub fn get_flag(&self, bit: CpuStatus) -> bool {
        self.p & (1 << (bit as u8)) > 0
    }
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!(
            "a: {}, x: {}, y: {}, pc: {}, p: {}",
            self.a, self.x, self.y, self.pc, self.p
        );

        write!(f, "[C: {}, ", self.get_flag(CpuStatus::Carry)).unwrap();
        write!(f, "Z: {}, ", self.get_flag(CpuStatus::Zero)).unwrap();
        write!(f, "I: {}, ", self.get_flag(CpuStatus::InterruptDisable)).unwrap();
        write!(f, "D: {}, ", self.get_flag(CpuStatus::Decimal)).unwrap();
        write!(f, "O: {}, ", self.get_flag(CpuStatus::Overflow)).unwrap();
        writeln!(f, "N: {}]", self.get_flag(CpuStatus::Negative))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_flag() {
        let mut cpu = Cpu::new();

        cpu.set_flag(CpuStatus::Carry, true);
        assert_eq!(cpu.p, 1);
        cpu.set_flag(CpuStatus::Overflow, true);
        assert_eq!(cpu.p, 0b01000001);
        cpu.set_flag(CpuStatus::Carry, false);
        cpu.set_flag(CpuStatus::Decimal, false);
        assert_eq!(cpu.p, 0b01000000);
    }

    #[test]
    fn test_get_flag() {
        let mut cpu = Cpu::new();

        cpu.set_flag(CpuStatus::Carry, true);
        assert!(cpu.get_flag(CpuStatus::Carry));
        assert!(!cpu.get_flag(CpuStatus::Overflow));
    }
}

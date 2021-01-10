use crate::cartridge::rom::NESRom;

use super::{
    address_mode::{AddressMode, AddressModeValue},
    bus::Bus,
};

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

#[repr(u16)]
#[derive(Clone, Copy)]
pub enum InterruptVector {
    Nmi = 0xFFFA,
    Reset = 0xFFFC,
    Irq = 0xFFFE,
}

pub struct Cpu {
    // 6502 CPU Registers
    pub bus: Bus,
    pub a: u8, // Accumulator
    pub x: u8, // X register (index)
    pub y: u8, // Y register (index)
    pc: u16,   // Program counter
    sp: u8,    // Stack pointer
    p: u8,     // Status register
}

impl Cpu {
    pub fn new(cartridge: Box<NESRom>) -> Cpu {
        Cpu {
            bus: Bus::new(cartridge),
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            p: 0,
        }
    }

    pub fn reset(&mut self) {
        self.sp = 0xFF;
        self.p = 0x34;
        self.interrupt(InterruptVector::Reset);
    }

    pub fn execute_next_opcode(&mut self) {
        let opcode = self.next_byte();
        self.execute(opcode);
    }

    pub fn push_stack(&mut self, value: u8) {
        self.bus.write(STACK_PAGE + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    pub fn push_stack_16(&mut self, value: u16) {
        self.push_stack((value >> 8) as u8);
        self.push_stack((value & 0xFF) as u8);
    }

    pub fn pop_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.bus.read(STACK_PAGE + self.sp as u16)
    }

    fn next_byte(&mut self) -> u8 {
        let value = self.bus.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    fn next_word(&mut self) -> u16 {
        let low_byte = self.next_byte() as u16;
        let high_byte = self.next_byte() as u16;

        low_byte | (high_byte << 8)
    }

    pub fn pop_stack_16(&mut self) -> u16 {
        let low_byte = self.pop_stack() as u16;
        let high_byte = self.pop_stack() as u16;

        low_byte | (high_byte << 8)
    }

    pub fn set_flag(&mut self, bit: CpuStatus, is_set: bool) {
        self.p = (self.p & !(1 << bit as u8)) | (u8::from(is_set) << bit as u8);
    }

    pub fn get_flag(&self, bit: CpuStatus) -> bool {
        self.p & (1 << (bit as u8)) > 0
    }

    pub fn operand_address(&mut self, mode: AddressMode) -> AddressModeValue {
        match mode {
            AddressMode::ZeroPage => self.zero_page_read(),
            AddressMode::ZeroPageIndexedX => self.zero_page_index_read(self.x),
            AddressMode::ZeroPageIndexedY => self.zero_page_index_read(self.y),
            AddressMode::Absolute => self.absolute_read(),
            AddressMode::AbsoluteIndexedX => self.absolute_index_read(self.x),
            AddressMode::AbsoluteIndexedY => self.absolute_index_read(self.y),
            AddressMode::IndexedIndirect => self.index_indirect_read(),
            AddressMode::IndirectIndexed => self.indirect_index_read(),
            AddressMode::Immediate => AddressModeValue::Immediate(self.next_byte()),
        }
    }

    fn read_operand(&mut self, mode: AddressMode) -> u8 {
        match self.operand_address(mode) {
            AddressModeValue::Absolute(x) => self.bus.read(x),
            AddressModeValue::Immediate(x) => x,
            val => panic!("Invalid address mode value: {:#?}", val),
        }
    }

    fn zero_page_index_read(&mut self, register: u8) -> AddressModeValue {
        let offset = self.next_byte() as u16;
        AddressModeValue::Absolute((offset + register as u16) % 256)
    }

    fn zero_page_read(&mut self) -> AddressModeValue {
        AddressModeValue::Absolute(self.next_byte() as u16)
    }

    fn absolute_read(&mut self) -> AddressModeValue {
        AddressModeValue::Absolute(self.next_word())
    }

    fn absolute_index_read(&mut self, register: u8) -> AddressModeValue {
        let offset = self.next_word();
        AddressModeValue::Absolute(offset + register as u16)
    }

    fn index_indirect_read(&mut self) -> AddressModeValue {
        let offset = self.next_byte() as u16 + self.x as u16;

        let low_byte = self.bus.read(offset & 0xFF) as u16;
        let high_byte = self.bus.read((offset + 1) % 0xFF) as u16;

        AddressModeValue::Absolute(low_byte | high_byte << 8)
    }

    fn indirect_index_read(&mut self) -> AddressModeValue {
        let arg = self.next_byte() as u16;

        let low_byte = self.bus.read(arg as u16) as u16;
        let high_byte = self.bus.read((arg + u16::from(self.x) + 1) & 0xFF) as u16;

        AddressModeValue::Absolute(low_byte | high_byte << 8)
    }

    fn interrupt(&mut self, interrupt: InterruptVector) {
        self.pc = self.bus.read_word(interrupt as u16);
        self.pc = 0xC000;
    }

    fn branch(&mut self, should_branch: bool) {
        // Pull the offset as a signed number
        let mut offset = self.read_operand(AddressMode::Immediate);

        println!("BPL #{:02x}", offset);

        if should_branch {
            self.pc = if offset & 0x80 > 0 {
                // Invert the 2s compliment
                offset = (offset ^ 0xFF) + 1;
                self.pc.wrapping_sub(offset as u16)
            } else {
                self.pc.wrapping_add(offset as u16)
            }
        }
    }

    fn execute(&mut self, opcode: u8) {
        println!("Execute: 0x{:02x}", opcode);
        match opcode {
            0x10 => self.bpl(),
            0x20 => self.jsr(),
            0x38 => self.sec(),
            0x4c => self.jmp(AddressMode::Absolute),
            0x78 => self.sei(),
            0x86 => self.stx(AddressMode::ZeroPage),
            0x9a => self.txs(),
            0xa2 => self.ldx(AddressMode::Immediate),
            0xad => self.lda(AddressMode::Absolute),
            0xd8 => self.cld(),
            0xea => self.nop(),
            _ => panic!("Unimplemented: 0x{:x}", opcode),
        }
    }

    // Branch on PLus
    fn bpl(&mut self) {
        self.branch(!self.get_flag(CpuStatus::Negative));
    }

    fn jmp(&mut self, mode: AddressMode) {
        self.pc = self.operand_address(mode).address();
    }

    fn jsr(&mut self) {
        let target_address = self.operand_address(AddressMode::Absolute);
        let return_address = self.pc.wrapping_sub(1);

        self.push_stack_16(return_address);
        self.pc = target_address.address();
    }

    fn lda(&mut self, mode: AddressMode) {
        let operand = self.read_operand(mode);
        self.set_flag(CpuStatus::Zero, operand == 0);
        self.set_flag(CpuStatus::Negative, operand > 0b10000000);
        self.a = operand;
    }

    // LoaD X
    fn ldx(&mut self, mode: AddressMode) {
        let operand = self.read_operand(mode);
        self.set_flag(CpuStatus::Zero, operand == 0);
        self.set_flag(CpuStatus::Negative, operand > 0b10000000);

        self.x = operand;
    }

    // Tranfer X to Stack pointer
    fn txs(&mut self) {
        let result = self.x;
        self.sp = result;
    }

    fn stx(&mut self, mode: AddressMode) {
        let address = match self.operand_address(mode) {
            AddressModeValue::Absolute(address) => address,
            v => panic!("Invalid address mode: {:#?}", v),
        };

        self.bus.write(address, self.x);
    }

    fn sec(&mut self) {
        self.set_flag(CpuStatus::Carry, true);
    }

    // CLear Decimal
    fn cld(&mut self) {
        self.set_flag(CpuStatus::Decimal, false);
    }

    // SEt Interrupt
    fn sei(&mut self) {
        self.set_flag(CpuStatus::InterruptDisable, true);
    }

    fn nop(&mut self) {}
}

impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!(
            "a: 0x{:2x}, x: 0x{:2x}, y: 0x{:2x}, pc: 0x{:4x}, p: 0x{:2x}, sp: 0x{:2x}",
            self.a, self.x, self.y, self.pc, self.p, self.sp
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
    use crate::cartridge::rom::NESRom;

    use super::*;

    #[test]
    fn test_set_flag() {
        let cartridge = Box::new(NESRom::from_file("./priv/nestest.nes").unwrap());
        let mut cpu = Cpu::new(cartridge);

        cpu.set_flag(CpuStatus::Carry, true);
        assert_eq!(cpu.p, 0b00110101);
        cpu.set_flag(CpuStatus::Overflow, true);
        assert_eq!(cpu.p, 0b01110101);
        cpu.set_flag(CpuStatus::Carry, false);
        cpu.set_flag(CpuStatus::Decimal, false);
        assert_eq!(cpu.p, 0b01110100);
    }

    #[test]
    fn test_get_flag() {
        let cartridge = Box::new(NESRom::from_file("./priv/nestest.nes").unwrap());
        let mut cpu = Cpu::new(cartridge);

        cpu.set_flag(CpuStatus::Carry, true);
        assert!(cpu.get_flag(CpuStatus::Carry));
        assert!(!cpu.get_flag(CpuStatus::Overflow));
    }

    #[test]
    fn test_stack() {
        let cartridge = Box::new(NESRom::from_file("./priv/nestest.nes").unwrap());
        let mut cpu = Cpu::new(cartridge);

        cpu.push_stack(1);
        cpu.push_stack(2);
        cpu.push_stack(3);
        assert_eq!(cpu.pop_stack(), 3);
        assert_eq!(cpu.pop_stack(), 2);
        assert_eq!(cpu.pop_stack(), 1);
    }

    #[test]
    fn test_stack_16() {
        let cartridge = Box::new(NESRom::from_file("./priv/nestest.nes").unwrap());
        let mut cpu = Cpu::new(cartridge);

        cpu.push_stack_16(0xFFFF);
        cpu.push_stack(0x12);
        cpu.push_stack_16(0xFEFE);
        cpu.push_stack_16(0xFAFA);
        assert_eq!(cpu.pop_stack_16(), 0xFAFA);
        assert_eq!(cpu.pop_stack_16(), 0xFEFE);
        assert_eq!(cpu.pop_stack(), 0x12);
        assert_eq!(cpu.pop_stack_16(), 0xFFFF);
    }
}

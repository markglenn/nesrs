use std::fmt;

use super::{
    address_mode::{AddressMode, AddressModeValue},
    cpu::Cpu,
    cpu::CpuStatus,
};

type OpcodeFunction = fn(&mut Cpu, AddressMode);
#[allow(dead_code)]
pub struct OpCode<'a> {
    name: &'a str,
    code: u8,
    func: OpcodeFunction,
    address_mode: AddressMode,
    cycles: u8,
}

impl OpCode<'_> {}

impl fmt::Debug for OpCode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{:02X}\t{}\t{:#?}",
            self.code, self.name, self.address_mode
        )
    }
}

pub fn execute(cpu: &mut Cpu, code: u8) {
    let opcode = &OPCODES[code as usize];

    print!(
        "{:#?}\t{:04X}  {:02X} {} ",
        cpu,
        cpu.pc - 1,
        code,
        opcode.name
    );

    (opcode.func)(cpu, opcode.address_mode);
    println!("");
}

#[allow(dead_code)]
pub static OPCODES: [OpCode; 0x100] = [
    // 0x00 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x00,
    },
    // 0x01 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::IndirectX,
        cycles: 6,
        code: 0x01,
    },
    // 0x02 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x03 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x04 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x05 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::ZeroPage,
        cycles: 3,
        code: 0x05,
    },
    // 0x06 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x07 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x08 -
    OpCode {
        name: "PHP",
        func: php,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x08,
    },
    // 0x09 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::Immediate,
        cycles: 0,
        code: 2,
    },
    // 0x0A -
    OpCode {
        name: "ASL",
        func: asl_a,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x0A,
    },
    // 0x0B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x0C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x0D -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0x0D,
    },
    // 0x0E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x0F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x10 -
    OpCode {
        name: "BPL",
        func: bpl,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x10,
    },
    // 0x11 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::IndirectY,
        cycles: 5,
        code: 0x11,
    },
    // 0x12 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x13 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x14 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x15 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::ZeroPageIndexedX,
        cycles: 4,
        code: 0x15,
    },
    // 0x16 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x17 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x18 -
    OpCode {
        name: "CLC",
        func: clc,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x18,
    },
    // 0x19 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 4,
        code: 0x19,
    },
    // 0x1A -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x1B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x1C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x1D -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::AbsoluteIndexedX,
        cycles: 4,
        code: 0x1D,
    },
    // 0x1E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x1F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x20 -
    OpCode {
        name: "JSR",
        func: jsr,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x20,
    },
    // 0x21 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::IndirectX,
        cycles: 6,
        code: 0x21,
    },
    // 0x22 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x23 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x24 -
    OpCode {
        name: "BIT",
        func: bit,
        address_mode: AddressMode::ZeroPage,
        cycles: 0,
        code: 0x24,
    },
    // 0x25 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::ZeroPage,
        cycles: 3,
        code: 0x25,
    },
    // 0x26 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x27 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x28 -
    OpCode {
        name: "PLP",
        func: plp,
        address_mode: AddressMode::Implied,
        cycles: 4,
        code: 0x28,
    },
    // 0x29 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::Immediate,
        cycles: 0,
        code: 0x29,
    },
    // 0x2A -
    OpCode {
        name: "ROL",
        func: rol_a,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x2A,
    },
    // 0x2B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x2C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x2D -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0x2D,
    },
    // 0x2E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x2F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x30 -
    OpCode {
        name: "BMI",
        func: bmi,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0x30,
    },
    // 0x31 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::IndirectY,
        cycles: 5,
        code: 0x31,
    },
    // 0x32 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x33 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x34 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x35 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::ZeroPageIndexedX,
        cycles: 4,
        code: 0x35,
    },
    // 0x36 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x37 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x38 -
    OpCode {
        name: "SEC",
        func: sec,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x38,
    },
    // 0x39 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 4,
        code: 0x39,
    },
    // 0x3A -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x3B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x3C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x3D -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::AbsoluteIndexedX,
        cycles: 4,
        code: 0x3D,
    },
    // 0x3E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x3F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x40 -
    OpCode {
        name: "RTI",
        func: rti,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x40,
    },
    // 0x41 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::IndirectX,
        cycles: 6,
        code: 0x41,
    },
    // 0x42 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x43 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x44 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x45 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::ZeroPage,
        cycles: 3,
        code: 0x45,
    },
    // 0x46 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x47 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x48 -
    OpCode {
        name: "PHA",
        func: pha,
        address_mode: AddressMode::Implied,
        cycles: 3,
        code: 0x48,
    },
    // 0x49 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::Immediate,
        cycles: 0,
        code: 0x49,
    },
    // 0x4A -
    OpCode {
        name: "LSR",
        func: lsr_a,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x4A,
    },
    // 0x4B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x4C -
    OpCode {
        name: "JMP",
        func: jmp,
        address_mode: AddressMode::Absolute,
        cycles: 0,
        code: 0x4c,
    },
    // 0x4D -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0x4D,
    },
    // 0x4E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x4F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x50 -
    OpCode {
        name: "BVC",
        func: bvc,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x50,
    },
    // 0x51 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::IndirectY,
        cycles: 5,
        code: 0x51,
    },
    // 0x52 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x53 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x54 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x55 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::ZeroPageIndexedX,
        cycles: 4,
        code: 0x55,
    },
    // 0x56 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x57 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x58 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x59 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 4,
        code: 0x59,
    },
    // 0x5A -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x5B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x5C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x5D -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::AbsoluteIndexedX,
        cycles: 4,
        code: 0x5D,
    },
    // 0x5E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x5F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x60 -
    OpCode {
        name: "RTS",
        func: rts,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x60,
    },
    // 0x61 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::IndirectX,
        cycles: 6,
        code: 0x61,
    },
    // 0x62 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x63 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x64 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x65 -
    OpCode {
        name: "ADC",
        func: invalid,
        address_mode: AddressMode::ZeroPage,
        cycles: 3,
        code: 0x65,
    },
    // 0x66 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x67 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x68 -
    OpCode {
        name: "PLA",
        func: pla,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x68,
    },
    // 0x69 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::Immediate,
        cycles: 0,
        code: 0,
    },
    // 0x6A -
    OpCode {
        name: "ROR",
        func: ror_a,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x6A,
    },
    // 0x6B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x6C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x6D -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0x6D,
    },
    // 0x6E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x6F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x70 -
    OpCode {
        name: "BVS",
        func: bvs,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x70,
    },
    // 0x71 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::IndirectY,
        cycles: 5,
        code: 0x71,
    },
    // 0x72 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x73 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x74 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x75 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::ZeroPageIndexedX,
        cycles: 4,
        code: 0x75,
    },
    // 0x76 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x77 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x78 -
    OpCode {
        name: "SEI",
        func: sei,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x78,
    },
    // 0x79 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 4,
        code: 0x79,
    },
    // 0x7A -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x7B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x7C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x7D -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::AbsoluteIndexedX,
        cycles: 4,
        code: 0x7D,
    },
    // 0x7E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x7F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x80 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x81 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::IndirectX,
        cycles: 6,
        code: 0x81,
    },
    // 0x82 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x83 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x84 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x85 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::ZeroPage,
        cycles: 0,
        code: 0x85,
    },
    // 0x86 -
    OpCode {
        name: "STX",
        func: stx,
        address_mode: AddressMode::ZeroPage,
        cycles: 0,
        code: 0x86,
    },
    // 0x87 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x88 -
    OpCode {
        name: "DEY",
        func: dey,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0x88,
    },
    // 0x89 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x8A -
    OpCode {
        name: "TXA",
        func: txa,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0x8A,
    },
    // 0x8B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x8C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x8D -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0x8D,
    },
    // 0x8E -
    OpCode {
        name: "STX",
        func: stx,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0x8E,
    },
    // 0x8F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x90 -
    OpCode {
        name: "BCC",
        func: bcc,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0x90,
    },
    // 0x91 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::IndirectY,
        cycles: 6,
        code: 0x91,
    },
    // 0x92 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x93 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x94 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x95 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::ZeroPageIndexedX,
        cycles: 4,
        code: 0x95,
    },
    // 0x96 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x97 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x98 -
    OpCode {
        name: "TYA",
        func: tya,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0x98,
    },
    // 0x99 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 5,
        code: 0x99,
    },
    // 0x9A -
    OpCode {
        name: "TXS",
        func: txs,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0x9A,
    },
    // 0x9B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x9C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x9D -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::AbsoluteIndexedX,
        cycles: 5,
        code: 0x9D,
    },
    // 0x9E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0x9F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xA0 -
    OpCode {
        name: "LDY",
        func: ldy,
        address_mode: AddressMode::Immediate,
        cycles: 2,
        code: 0xA0,
    },
    // 0xA1 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::IndirectX,
        cycles: 6,
        code: 0xA1,
    },
    // 0xA2 -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::Immediate,
        cycles: 0,
        code: 0xA2,
    },
    // 0xA3 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xA4 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xA5 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::ZeroPage,
        cycles: 3,
        code: 0xA5,
    },
    // 0xA6 -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::ZeroPage,
        cycles: 3,
        code: 0xA6,
    },
    // 0xA7 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xA8 -
    OpCode {
        name: "TAY",
        func: tay,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0xA8,
    },
    // 0xA9 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::Immediate,
        cycles: 0,
        code: 0xA9,
    },
    // 0xAA -
    OpCode {
        name: "TAX",
        func: tax,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0xAA,
    },
    // 0xAB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xAC -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xAD -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0xAD,
    },
    // 0xAE -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0xAE,
    },
    // 0xAF -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xB0 -
    OpCode {
        name: "BCS",
        func: bcs,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0xB0,
    },
    // 0xB1 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::IndirectY,
        cycles: 5,
        code: 0xB1,
    },
    // 0xB2 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xB3 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xB4 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xB5 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::ZeroPageIndexedX,
        cycles: 4,
        code: 0xB5,
    },
    // 0xB6 -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::ZeroPageIndexedY,
        cycles: 4,
        code: 0xB6,
    },
    // 0xB7 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xB8 -
    OpCode {
        name: "CLV",
        func: clv,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0xB8,
    },
    // 0xB9 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 4,
        code: 0xB9,
    },
    // 0xBA -
    OpCode {
        name: "TSX",
        func: tsx,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0xBA,
    },
    // 0xBB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xBC -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xBD -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::AbsoluteIndexedX,
        cycles: 4,
        code: 0xBD,
    },
    // 0xBE -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 4,
        code: 0xBE,
    },
    // 0xBF -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xC0 -
    OpCode {
        name: "CPY",
        func: cpy,
        address_mode: AddressMode::Immediate,
        cycles: 0,
        code: 0xC0,
    },
    // 0xC1 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::IndirectX,
        cycles: 6,
        code: 0xC1,
    },
    // 0xC2 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xC3 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xC4 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xC5 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::ZeroPage,
        cycles: 3,
        code: 0xC5,
    },
    // 0xC6 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xC7 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xC8 -
    OpCode {
        name: "INY",
        func: iny,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0xC8,
    },
    // 0xC9 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::Immediate,
        cycles: 0,
        code: 0xC9,
    },
    // 0xCA -
    OpCode {
        name: "DEX",
        func: dex,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0xCA,
    },
    // 0xCB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xCC -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xCD -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::Absolute,
        cycles: 4,
        code: 0xCD,
    },
    // 0xCE -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xCF -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xD0 -
    OpCode {
        name: "BNE",
        func: bne,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0xD0,
    },
    // 0xD1 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::IndirectY,
        cycles: 5,
        code: 0xD1,
    },
    // 0xD2 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xD3 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xD4 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xD5 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::ZeroPageIndexedX,
        cycles: 4,
        code: 0xD5,
    },
    // 0xD6 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xD7 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xD8 -
    OpCode {
        name: "CLD",
        func: cld,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0xD8,
    },
    // 0xD9 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 4,
        code: 0xD9,
    },
    // 0xDA -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xDB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xDC -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xDD -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::AbsoluteIndexedX,
        cycles: 4,
        code: 0xDD,
    },
    // 0xDE -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xDF -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xE0 -
    OpCode {
        name: "CPX",
        func: cpx,
        address_mode: AddressMode::Immediate,
        cycles: 2,
        code: 0xE0,
    },
    // 0xE1 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::IndirectX,
        cycles: 0,
        code: 0,
    },
    // 0xE2 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xE3 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xE4 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xE5 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::ZeroPage,
        cycles: 0,
        code: 0,
    },
    // 0xE6 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xE7 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xE8 - INX
    OpCode {
        name: "INX",
        func: inx,
        address_mode: AddressMode::Implied,
        cycles: 2,
        code: 0xE8,
    },
    // 0xE9 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::Immediate,
        cycles: 2,
        code: 0xE9,
    },
    // 0xEA -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0xEA,
    },
    // 0xEB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xEC -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xED -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::Absolute,
        cycles: 0,
        code: 0,
    },
    // 0xEE -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xEF -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xF0 -
    OpCode {
        name: "BEQ",
        func: beq,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0xF0,
    },
    // 0xF1 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::IndirectY,
        cycles: 0,
        code: 0,
    },
    // 0xF2 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xF3 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xF4 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xF5 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::ZeroPageIndexedX,
        cycles: 0,
        code: 0,
    },
    // 0xF6 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xF7 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xF8 -
    OpCode {
        name: "SED",
        func: sed,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0xF8,
    },
    // 0xF9 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::AbsoluteIndexedY,
        cycles: 0,
        code: 0,
    },
    // 0xFA -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xFB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xFC -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xFD -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::AbsoluteIndexedX,
        cycles: 0,
        code: 0,
    },
    // 0xFE -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
    // 0xFF -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
        cycles: 0,
        code: 0,
    },
];

// JuMP
fn jmp(cpu: &mut Cpu, mode: AddressMode) {
    cpu.pc = cpu.operand_address(mode).address();
}

// Jump SubRoutine
fn jsr(cpu: &mut Cpu, _mode: AddressMode) {
    let target_address = cpu.operand_address(AddressMode::Absolute);
    let return_address = cpu.pc - 1;

    print!("${:04X}", target_address.address());

    cpu.push_stack_16(return_address);
    cpu.pc = target_address.address();
}

/* Load/Store Functions */

fn lda(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);
    print!(" = {:02X}", operand);
    set_zero_and_negative(cpu, operand);

    cpu.a = operand;
}

fn sta(cpu: &mut Cpu, mode: AddressMode) {
    let address = cpu.operand_address(mode).address();
    cpu.bus.write(address, cpu.a);
}

fn ldx(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);
    set_zero_and_negative(cpu, operand);
    cpu.x = operand;
}

fn stx(cpu: &mut Cpu, mode: AddressMode) {
    let address = cpu.operand_address(mode).address();
    cpu.bus.write(address, cpu.x);
}

fn ldy(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);
    set_zero_and_negative(cpu, operand);
    cpu.y = operand;
}

fn sty(cpu: &mut Cpu, mode: AddressMode) {
    let address = cpu.operand_address(mode).address();
    cpu.bus.write(address, cpu.y);
}

/* Stack Instructions */

// PusH Processor status
fn php(cpu: &mut Cpu, _mode: AddressMode) {
    // Bits 4 & 5 are always set on PHP
    //   http://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag
    cpu.push_stack(cpu.p | 0b110000);
}

// PusH Accumulator
fn pha(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.push_stack(cpu.a);
}

// PuLl Processor status
fn plp(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.p = (cpu.pop_stack() & 0b11001111) | (cpu.p & 0b00110000);
}

// PuLl Accumulator
fn pla(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.a = cpu.pop_stack();
    set_zero_and_negative(cpu, cpu.a);
}

/* Branch Instructions */

// Branch on Carry Set
fn bcs(cpu: &mut Cpu, _mode: AddressMode) {
    let carry = cpu.get_flag(CpuStatus::Carry);
    branch(cpu, carry);
}

// Branch on Carry Clear
fn bcc(cpu: &mut Cpu, _mode: AddressMode) {
    let carry = cpu.get_flag(CpuStatus::Carry);
    branch(cpu, !carry);
}

// Branch on EQual
fn beq(cpu: &mut Cpu, _mode: AddressMode) {
    branch(cpu, cpu.get_flag(CpuStatus::Zero));
}

// Branch on Not Equal
fn bne(cpu: &mut Cpu, _mode: AddressMode) {
    branch(cpu, !cpu.get_flag(CpuStatus::Zero));
}

// Branch on oVerflow Set
fn bvs(cpu: &mut Cpu, _mode: AddressMode) {
    branch(cpu, cpu.get_flag(CpuStatus::Overflow));
}

// Branch on oVerflow Clear
fn bvc(cpu: &mut Cpu, _mode: AddressMode) {
    branch(cpu, !cpu.get_flag(CpuStatus::Overflow));
}

// Branch on PLus
fn bpl(cpu: &mut Cpu, _mode: AddressMode) {
    branch(cpu, !cpu.get_flag(CpuStatus::Negative));
}

// Branch on MInus
fn bmi(cpu: &mut Cpu, _mode: AddressMode) {
    branch(cpu, cpu.get_flag(CpuStatus::Negative));
}

// ReTurn from Subroutine
fn rts(cpu: &mut Cpu, _mode: AddressMode) {
    let return_address = cpu.pop_stack_16() + 1;
    cpu.pc = return_address;
}

fn rti(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.p = (cpu.pop_stack() & 0b11011111) | (cpu.p & 0b00100000);
    cpu.pc = cpu.pop_stack_16();
}

/* Logical Operations */

// AND
fn and(cpu: &mut Cpu, mode: AddressMode) {
    let value = cpu.a & read_operand(cpu, mode);
    set_zero_and_negative(cpu, value);
    cpu.a = value;
}

// OR Accumulator
fn ora(cpu: &mut Cpu, mode: AddressMode) {
    let value = cpu.a | read_operand(cpu, mode);
    set_zero_and_negative(cpu, value);
    cpu.a = value;
}

// Exclusive OR
fn eor(cpu: &mut Cpu, mode: AddressMode) {
    cpu.a = cpu.a ^ read_operand(cpu, mode);
    set_zero_and_negative(cpu, cpu.a);
}

/* Arithmetic Operations */

// ADd with Carry
fn adc(cpu: &mut Cpu, mode: AddressMode) {
    let a = cpu.a;
    let operand = read_operand(cpu, mode);
    let mut result = (a as u16).wrapping_add(operand as u16);

    if cpu.get_flag(CpuStatus::Carry) {
        result = result.wrapping_add(1);
    }

    cpu.a = result as u8;
    set_zero_and_negative(cpu, result as u8);
    set_carry_and_overflow(cpu, a, operand, result);
}

// SuBtract with Carry
fn sbc(cpu: &mut Cpu, mode: AddressMode) {
    let a = cpu.a;
    let operand = !read_operand(cpu, mode);
    let mut result = (a as u16).wrapping_add(operand as u16);

    if cpu.get_flag(CpuStatus::Carry) {
        result = result.wrapping_add(1);
    }

    cpu.a = result as u8;
    set_zero_and_negative(cpu, result as u8);
    set_carry_and_overflow(cpu, a, operand, result);
}

// INcrement Y
fn iny(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.y = cpu.y.wrapping_add(1);
    set_zero_and_negative(cpu, cpu.y);
}

// DEcrement Y
fn dey(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.y = cpu.y.wrapping_sub(1);
    set_zero_and_negative(cpu, cpu.y);
}

// INcrement X
fn inx(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.x = cpu.x.wrapping_add(1);
    set_zero_and_negative(cpu, cpu.x);
}

// DEcrement X
fn dex(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.x = cpu.x.wrapping_sub(1);
    set_zero_and_negative(cpu, cpu.x);
}

/* Comparison Operations */

// CoMPare (with accumulator)
fn cmp(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);
    set_zero_and_negative(cpu, cpu.a.wrapping_sub(operand));
    cpu.set_flag(CpuStatus::Carry, cpu.a >= operand);
}

// ComPare X register
fn cpx(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);
    set_zero_and_negative(cpu, cpu.x.wrapping_sub(operand));
    cpu.set_flag(CpuStatus::Carry, cpu.x >= operand);
}

// ComPare Y register
fn cpy(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);
    set_zero_and_negative(cpu, cpu.y.wrapping_sub(operand));
    cpu.set_flag(CpuStatus::Carry, cpu.y >= operand);
}

/* Transfer Operations */

// Transfer A to X
fn tax(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.x = cpu.a;
    set_zero_and_negative(cpu, cpu.a);
}

// Transfer X to A
fn txa(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.a = cpu.x;
    set_zero_and_negative(cpu, cpu.a);
}

// Transfer A to Y
fn tay(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.y = cpu.a;
    set_zero_and_negative(cpu, cpu.a);
}

// Transfer Y to A
fn tya(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.a = cpu.y;
    set_zero_and_negative(cpu, cpu.a);
}

// Transfer Stack pointer to X
fn tsx(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.x = cpu.sp;
    set_zero_and_negative(cpu, cpu.sp);
}

// Transfer X to Stack pointer
fn txs(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.sp = cpu.x;
}

/* Bit Manipulation Operations */

fn lsr_a(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Carry, cpu.a & 1 != 0);
    cpu.a = cpu.a >> 1;
    set_zero_and_negative(cpu, cpu.a);
}

fn asl_a(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Carry, cpu.a & 0x80 > 0);
    cpu.a = cpu.a << 1;
    set_zero_and_negative(cpu, cpu.a);
}

fn ror_a(cpu: &mut Cpu, _mode: AddressMode) {
    let mut value = cpu.a >> 1;

    if cpu.get_flag(CpuStatus::Carry) {
        value |= 0x80;
    }

    cpu.set_flag(CpuStatus::Carry, cpu.a & 1 != 0);
    cpu.a = value;
    set_zero_and_negative(cpu, value);
}

fn rol_a(cpu: &mut Cpu, _mode: AddressMode) {
    let mut value = cpu.a << 1;

    if cpu.get_flag(CpuStatus::Carry) {
        value += 1;
    }

    cpu.set_flag(CpuStatus::Carry, cpu.a & 0x80 != 0);
    cpu.a = value;
    set_zero_and_negative(cpu, value);
}

/* Miscellaneous Operations */

fn bit(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);

    cpu.set_flag(CpuStatus::Zero, operand & cpu.a == 0);
    cpu.set_flag(CpuStatus::Negative, operand & 0b10000000 != 0);
    cpu.set_flag(CpuStatus::Overflow, operand & 0b01000000 != 0);
}

// No OPeration
fn nop(_cpu: &mut Cpu, _mode: AddressMode) {}

// SEt Carry
fn sec(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Carry, true);
}

// CLear Carry
fn clc(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Carry, false);
}

// SEt Interrupt
fn sei(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::InterruptDisable, true);
}

// CLear Interrupt
fn cli(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::InterruptDisable, false);
}

// CLear oVerflow
fn clv(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Overflow, false);
}

// SEt Decimal
fn sed(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Decimal, true);
}

// CLear Decimal
fn cld(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Decimal, false);
}

fn invalid(_cpu: &mut Cpu, _mode: AddressMode) {
    panic!("Invalid operation");
}

fn branch(cpu: &mut Cpu, success: bool) {
    // Pull the offset as a signed number
    let offset = read_operand(cpu, AddressMode::Immediate) as i8 as u16;

    if success {
        cpu.pc = cpu.pc.wrapping_add(offset as u16);
    }
}
fn read_operand(cpu: &mut Cpu, mode: AddressMode) -> u8 {
    match cpu.operand_address(mode) {
        AddressModeValue::Absolute(x) => {
            let result = cpu.bus.read(x);
            print!("${:02X}", result);
            result
        }
        AddressModeValue::Immediate(x) => {
            print!("#${:02X}", x);
            x
        }
        val => panic!("Invalid address mode value: {:#?}", val),
    }
}

fn set_zero_and_negative(cpu: &mut Cpu, value: u8) {
    cpu.set_flag(CpuStatus::Zero, value == 0);
    cpu.set_flag(CpuStatus::Negative, value >> 7 != 0);
}

fn set_carry_and_overflow(cpu: &mut Cpu, left: u8, right: u8, result: u16) {
    cpu.set_flag(CpuStatus::Carry, result > 0xFF);
    let r = result as u8;
    cpu.set_flag(CpuStatus::Overflow, (left ^ r) & (right ^ r) & 0x80 != 0);
}

#[cfg(test)]
mod test {}

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
    func: OpcodeFunction,
    address_mode: AddressMode,
}

impl OpCode<'_> {}

impl fmt::Debug for OpCode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{:#?}", self.name, self.address_mode)
    }
}

pub fn execute(cpu: &mut Cpu, code: u8) {
    let opcode = &OPCODES[code as usize];

    print_debug(code, opcode, cpu);

    (opcode.func)(cpu, opcode.address_mode);
}

fn print_debug(code: u8, opcode: &OpCode, cpu: &mut Cpu) {
    let bytes = match opcode.address_mode {
        AddressMode::Absolute
        | AddressMode::AbsoluteIndexedX(false)
        | AddressMode::AbsoluteIndexedY(false)
        | AddressMode::Indirect => {
            format!(
                "{:02X} {:02X} {:02X}",
                code,
                cpu.bus.unclocked_read(cpu.pc),
                cpu.bus.unclocked_read(cpu.pc + 1),
            )
        }
        AddressMode::Implied => format!("{:02X}", code),
        _ => format!("{:02X} {:02X}", code, cpu.bus.unclocked_read(cpu.pc)),
    };

    let operation = match opcode.address_mode {
        AddressMode::Immediate => {
            format!("{:3} #${:02X}", opcode.name, cpu.bus.unclocked_read(cpu.pc))
        }
        AddressMode::Indirect => {
            let address = cpu.bus.unclocked_read_word(cpu.pc);
            let memory_value = cpu.bus.unclocked_read_word(address);
            format!(
                "{:3} (${:04X}) = {:04X}",
                opcode.name, address, memory_value
            )
        }
        AddressMode::Absolute => {
            let address = cpu.bus.unclocked_read_word(cpu.pc);
            format!(
                "{:3} ${:04X} = {:02X}",
                opcode.name,
                address,
                cpu.bus.unclocked_read(address)
            )
        }
        AddressMode::Offset => format!(
            "{:3} ${:04X}",
            opcode.name,
            cpu.pc
                .wrapping_add(cpu.bus.unclocked_read(cpu.pc) as i8 as u16)
                .wrapping_add(1)
        ),
        AddressMode::ZeroPage => {
            let operand = cpu.bus.unclocked_read(cpu.pc);
            let value = cpu.bus.unclocked_read(operand as u16);
            format!("{:3} ${:02X} = ${:02X}", opcode.name, operand, value)
        }
        AddressMode::Implied => format!("{:3}", opcode.name),
        _ => format!("{:3} ${:02X}", opcode.name, cpu.bus.unclocked_read(cpu.pc)),
    };

    println!(
        "{:04X}  {:8}  {:30}  {:#?}",
        cpu.pc - 1,
        bytes,
        operation,
        cpu
    );
}

#[allow(dead_code)]
pub static OPCODES: [OpCode; 0x100] = [
    // 0x00 -
    OpCode {
        name: "BRK",
        func: brk,
        address_mode: AddressMode::Implied,
    },
    // 0x01 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::IndirectX,
    },
    // 0x02 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x03 -
    OpCode {
        name: "SLO",
        func: slo,
        address_mode: AddressMode::IndirectX,
    },
    // 0x04 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x05 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x06 -
    OpCode {
        name: "ASL",
        func: asl,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x07 -
    OpCode {
        name: "SLO",
        func: slo,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x08 -
    OpCode {
        name: "PHP",
        func: php,
        address_mode: AddressMode::Implied,
    },
    // 0x09 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::Immediate,
    },
    // 0x0A -
    OpCode {
        name: "ASL",
        func: asl_a,
        address_mode: AddressMode::Implied,
    },
    // 0x0B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x0C -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Absolute,
    },
    // 0x0D -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::Absolute,
    },
    // 0x0E -
    OpCode {
        name: "ASL",
        func: asl,
        address_mode: AddressMode::Absolute,
    },
    // 0x0F -
    OpCode {
        name: "SLO",
        func: slo,
        address_mode: AddressMode::Absolute,
    },
    // 0x10 -
    OpCode {
        name: "BPL",
        func: bpl,
        address_mode: AddressMode::Offset,
    },
    // 0x11 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0x12 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x13 -
    OpCode {
        name: "SLO",
        func: slo,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0x14 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x15 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x16 -
    OpCode {
        name: "ASL",
        func: asl,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x17 -
    OpCode {
        name: "SLO",
        func: slo,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x18 -
    OpCode {
        name: "CLC",
        func: clc,
        address_mode: AddressMode::Implied,
    },
    // 0x19 -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0x1A -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Implied,
    },
    // 0x1B -
    OpCode {
        name: "SLO",
        func: slo,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0x1C -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x1D -
    OpCode {
        name: "ORA",
        func: ora,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x1E -
    OpCode {
        name: "ASL",
        func: asl,
        address_mode: AddressMode::AbsoluteIndexedX(true),
    },
    // 0x1F -
    OpCode {
        name: "SLO",
        func: slo,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x20 -
    OpCode {
        name: "JSR",
        func: jsr,
        address_mode: AddressMode::Absolute,
    },
    // 0x21 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::IndirectX,
    },
    // 0x22 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x23 -
    OpCode {
        name: "RLA",
        func: rla,
        address_mode: AddressMode::IndirectX,
    },
    // 0x24 -
    OpCode {
        name: "BIT",
        func: bit,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x25 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x26 -
    OpCode {
        name: "ROL",
        func: rol,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x27 -
    OpCode {
        name: "RLA",
        func: rla,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x28 -
    OpCode {
        name: "PLP",
        func: plp,
        address_mode: AddressMode::Implied,
    },
    // 0x29 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::Immediate,
    },
    // 0x2A -
    OpCode {
        name: "ROL",
        func: rol_a,
        address_mode: AddressMode::Implied,
    },
    // 0x2B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x2C -
    OpCode {
        name: "BIT",
        func: bit,
        address_mode: AddressMode::Absolute,
    },
    // 0x2D -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::Absolute,
    },
    // 0x2E -
    OpCode {
        name: "ROL",
        func: rol,
        address_mode: AddressMode::Absolute,
    },
    // 0x2F -
    OpCode {
        name: "RLA",
        func: rla,
        address_mode: AddressMode::Absolute,
    },
    // 0x30 -
    OpCode {
        name: "BMI",
        func: bmi,
        address_mode: AddressMode::Offset,
    },
    // 0x31 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0x32 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x33 -
    OpCode {
        name: "RLA",
        func: rla,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0x34 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x35 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x36 -
    OpCode {
        name: "ROL",
        func: rol,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x37 -
    OpCode {
        name: "RLA",
        func: rla,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x38 -
    OpCode {
        name: "SEC",
        func: sec,
        address_mode: AddressMode::Implied,
    },
    // 0x39 -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0x3A -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Implied,
    },
    // 0x3B -
    OpCode {
        name: "RLA",
        func: rla,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0x3C -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x3D -
    OpCode {
        name: "AND",
        func: and,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x3E -
    OpCode {
        name: "ROL",
        func: rol,
        address_mode: AddressMode::AbsoluteIndexedX(true),
    },
    // 0x3F -
    OpCode {
        name: "RLA",
        func: rla,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x40 -
    OpCode {
        name: "RTI",
        func: rti,
        address_mode: AddressMode::Implied,
    },
    // 0x41 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::IndirectX,
    },
    // 0x42 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x43 -
    OpCode {
        name: "SRE",
        func: sre,
        address_mode: AddressMode::IndirectX,
    },
    // 0x44 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x45 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x46 -
    OpCode {
        name: "LSR",
        func: lsr,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x47 -
    OpCode {
        name: "SRE",
        func: sre,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x48 -
    OpCode {
        name: "PHA",
        func: pha,
        address_mode: AddressMode::Implied,
    },
    // 0x49 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::Immediate,
    },
    // 0x4A -
    OpCode {
        name: "LSR",
        func: lsr_a,
        address_mode: AddressMode::Implied,
    },
    // 0x4B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x4C -
    OpCode {
        name: "JMP",
        func: jmp,
        address_mode: AddressMode::Absolute,
    },
    // 0x4D -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::Absolute,
    },
    // 0x4E -
    OpCode {
        name: "LSR",
        func: lsr,
        address_mode: AddressMode::Absolute,
    },
    // 0x4F -
    OpCode {
        name: "SRE",
        func: sre,
        address_mode: AddressMode::Absolute,
    },
    // 0x50 -
    OpCode {
        name: "BVC",
        func: bvc,
        address_mode: AddressMode::Offset,
    },
    // 0x51 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0x52 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x53 -
    OpCode {
        name: "SRE",
        func: sre,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0x54 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x55 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x56 -
    OpCode {
        name: "LSR",
        func: lsr,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x57 -
    OpCode {
        name: "SRE",
        func: sre,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x58 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x59 -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0x5A -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Implied,
    },
    // 0x5B -
    OpCode {
        name: "SRE",
        func: sre,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0x5C -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x5D -
    OpCode {
        name: "EOR",
        func: eor,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x5E -
    OpCode {
        name: "LSR",
        func: lsr,
        address_mode: AddressMode::AbsoluteIndexedX(true),
    },
    // 0x5F -
    OpCode {
        name: "SRE",
        func: sre,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x60 -
    OpCode {
        name: "RTS",
        func: rts,
        address_mode: AddressMode::Implied,
    },
    // 0x61 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::IndirectX,
    },
    // 0x62 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x63 -
    OpCode {
        name: "RRA",
        func: rra,
        address_mode: AddressMode::IndirectX,
    },
    // 0x64 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x65 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x66 -
    OpCode {
        name: "ROR",
        func: ror,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x67 -
    OpCode {
        name: "RRA",
        func: rra,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x68 -
    OpCode {
        name: "PLA",
        func: pla,
        address_mode: AddressMode::Implied,
    },
    // 0x69 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::Immediate,
    },
    // 0x6A -
    OpCode {
        name: "ROR",
        func: ror_a,
        address_mode: AddressMode::Implied,
    },
    // 0x6B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x6C -
    OpCode {
        name: "JMP",
        func: jmp,
        address_mode: AddressMode::Indirect,
    },
    // 0x6D -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::Absolute,
    },
    // 0x6E -
    OpCode {
        name: "ROR",
        func: ror,
        address_mode: AddressMode::Absolute,
    },
    // 0x6F -
    OpCode {
        name: "RRA",
        func: rra,
        address_mode: AddressMode::Absolute,
    },
    // 0x70 -
    OpCode {
        name: "BVS",
        func: bvs,
        address_mode: AddressMode::Offset,
    },
    // 0x71 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0x72 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x73 -
    OpCode {
        name: "RRA",
        func: rra,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0x74 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x75 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x76 -
    OpCode {
        name: "ROR",
        func: ror,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x77 -
    OpCode {
        name: "RRA",
        func: rra,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x78 -
    OpCode {
        name: "SEI",
        func: sei,
        address_mode: AddressMode::Implied,
    },
    // 0x79 -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0x7A -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Implied,
    },
    // 0x7B -
    OpCode {
        name: "RRA",
        func: rra,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0x7C -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x7D -
    OpCode {
        name: "ADC",
        func: adc,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x7E -
    OpCode {
        name: "ROR",
        func: ror,
        address_mode: AddressMode::AbsoluteIndexedX(true),
    },
    // 0x7F -
    OpCode {
        name: "RRA",
        func: rra,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0x80 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Immediate,
    },
    // 0x81 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::IndirectX,
    },
    // 0x82 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Immediate,
    },
    // 0x83 -
    OpCode {
        name: "SAX",
        func: sax,
        address_mode: AddressMode::IndirectX,
    },
    // 0x84 -
    OpCode {
        name: "STY",
        func: sty,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x85 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x86 -
    OpCode {
        name: "STX",
        func: stx,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x87 -
    OpCode {
        name: "SAX",
        func: sax,
        address_mode: AddressMode::ZeroPage,
    },
    // 0x88 -
    OpCode {
        name: "DEY",
        func: dey,
        address_mode: AddressMode::Implied,
    },
    // 0x89 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Immediate,
    },
    // 0x8A -
    OpCode {
        name: "TXA",
        func: txa,
        address_mode: AddressMode::Implied,
    },
    // 0x8B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x8C -
    OpCode {
        name: "STY",
        func: sty,
        address_mode: AddressMode::Absolute,
    },
    // 0x8D -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::Absolute,
    },
    // 0x8E -
    OpCode {
        name: "STX",
        func: stx,
        address_mode: AddressMode::Absolute,
    },
    // 0x8F -
    OpCode {
        name: "SAX",
        func: sax,
        address_mode: AddressMode::Absolute,
    },
    // 0x90 -
    OpCode {
        name: "BCC",
        func: bcc,
        address_mode: AddressMode::Offset,
    },
    // 0x91 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::IndirectY(true),
    },
    // 0x92 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x93 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x94 -
    OpCode {
        name: "STY",
        func: sty,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x95 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0x96 -
    OpCode {
        name: "STX",
        func: stx,
        address_mode: AddressMode::ZeroPageIndexedY,
    },
    // 0x97 -
    OpCode {
        name: "SAX",
        func: sax,
        address_mode: AddressMode::ZeroPageIndexedY,
    },
    // 0x98 -
    OpCode {
        name: "TYA",
        func: tya,
        address_mode: AddressMode::Implied,
    },
    // 0x99 -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::AbsoluteIndexedY(true),
    },
    // 0x9A -
    OpCode {
        name: "TXS",
        func: txs,
        address_mode: AddressMode::Implied,
    },
    // 0x9B -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x9C -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x9D -
    OpCode {
        name: "STA",
        func: sta,
        address_mode: AddressMode::AbsoluteIndexedX(true),
    },
    // 0x9E -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0x9F -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0xA0 -
    OpCode {
        name: "LDY",
        func: ldy,
        address_mode: AddressMode::Immediate,
    },
    // 0xA1 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::IndirectX,
    },
    // 0xA2 -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::Immediate,
    },
    // 0xA3 -
    OpCode {
        name: "LAX",
        func: lax,
        address_mode: AddressMode::IndirectX,
    },
    // 0xA4 -
    OpCode {
        name: "LDY",
        func: ldy,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xA5 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xA6 -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xA7 -
    OpCode {
        name: "LAX",
        func: lax,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xA8 -
    OpCode {
        name: "TAY",
        func: tay,
        address_mode: AddressMode::Implied,
    },
    // 0xA9 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::Immediate,
    },
    // 0xAA -
    OpCode {
        name: "TAX",
        func: tax,
        address_mode: AddressMode::Implied,
    },
    // 0xAB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0xAC -
    OpCode {
        name: "LDY",
        func: ldy,
        address_mode: AddressMode::Absolute,
    },
    // 0xAD -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::Absolute,
    },
    // 0xAE -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::Absolute,
    },
    // 0xAF -
    OpCode {
        name: "LAX",
        func: lax,
        address_mode: AddressMode::Absolute,
    },
    // 0xB0 -
    OpCode {
        name: "BCS",
        func: bcs,
        address_mode: AddressMode::Offset,
    },
    // 0xB1 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0xB2 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0xB3 -
    OpCode {
        name: "LAX",
        func: lax,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0xB4 -
    OpCode {
        name: "LDY",
        func: ldy,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xB5 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xB6 -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::ZeroPageIndexedY,
    },
    // 0xB7 -
    OpCode {
        name: "LAX",
        func: lax,
        address_mode: AddressMode::ZeroPageIndexedY,
    },
    // 0xB8 -
    OpCode {
        name: "CLV",
        func: clv,
        address_mode: AddressMode::Implied,
    },
    // 0xB9 -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0xBA -
    OpCode {
        name: "TSX",
        func: tsx,
        address_mode: AddressMode::Implied,
    },
    // 0xBB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0xBC -
    OpCode {
        name: "LDY",
        func: ldy,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0xBD -
    OpCode {
        name: "LDA",
        func: lda,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0xBE -
    OpCode {
        name: "LDX",
        func: ldx,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0xBF -
    OpCode {
        name: "LAX",
        func: lax,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0xC0 -
    OpCode {
        name: "CPY",
        func: cpy,
        address_mode: AddressMode::Immediate,
    },
    // 0xC1 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::IndirectX,
    },
    // 0xC2 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Immediate,
    },
    // 0xC3 -
    OpCode {
        name: "DCP",
        func: dcp,
        address_mode: AddressMode::IndirectX,
    },
    // 0xC4 -
    OpCode {
        name: "CPY",
        func: cpy,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xC5 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xC6 -
    OpCode {
        name: "DEC",
        func: dec,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xC7 -
    OpCode {
        name: "DCP",
        func: dcp,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xC8 -
    OpCode {
        name: "INY",
        func: iny,
        address_mode: AddressMode::Implied,
    },
    // 0xC9 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::Immediate,
    },
    // 0xCA -
    OpCode {
        name: "DEX",
        func: dex,
        address_mode: AddressMode::Implied,
    },
    // 0xCB -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0xCC -
    OpCode {
        name: "CPY",
        func: cpy,
        address_mode: AddressMode::Absolute,
    },
    // 0xCD -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::Absolute,
    },
    // 0xCE -
    OpCode {
        name: "DEC",
        func: dec,
        address_mode: AddressMode::Absolute,
    },
    // 0xCF -
    OpCode {
        name: "DCP",
        func: dcp,
        address_mode: AddressMode::Absolute,
    },
    // 0xD0 -
    OpCode {
        name: "BNE",
        func: bne,
        address_mode: AddressMode::Offset,
    },
    // 0xD1 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0xD2 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0xD3 -
    OpCode {
        name: "DCP",
        func: dcp,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0xD4 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xD5 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xD6 -
    OpCode {
        name: "DEC",
        func: dec,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xD7 -
    OpCode {
        name: "DCP",
        func: dcp,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xD8 -
    OpCode {
        name: "CLD",
        func: cld,
        address_mode: AddressMode::Implied,
    },
    // 0xD9 -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0xDA -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Implied,
    },
    // 0xDB -
    OpCode {
        name: "DCP",
        func: dcp,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0xDC -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0xDD -
    OpCode {
        name: "CMP",
        func: cmp,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0xDE -
    OpCode {
        name: "DEC",
        func: dec,
        address_mode: AddressMode::AbsoluteIndexedX(true),
    },
    // 0xDF -
    OpCode {
        name: "DCP",
        func: dcp,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0xE0 -
    OpCode {
        name: "CPX",
        func: cpx,
        address_mode: AddressMode::Immediate,
    },
    // 0xE1 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::IndirectX,
    },
    // 0xE2 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Immediate,
    },
    // 0xE3 -
    OpCode {
        name: "ISC",
        func: isc,
        address_mode: AddressMode::IndirectX,
    },
    // 0xE4 -
    OpCode {
        name: "CPX",
        func: cpx,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xE5 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xE6 -
    OpCode {
        name: "INC",
        func: inc,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xE7 -
    OpCode {
        name: "ISC",
        func: isc,
        address_mode: AddressMode::ZeroPage,
    },
    // 0xE8 - INX
    OpCode {
        name: "INX",
        func: inx,
        address_mode: AddressMode::Implied,
    },
    // 0xE9 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::Immediate,
    },
    // 0xEA -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Implied,
    },
    // 0xEB -
    OpCode {
        name: "SBC",
        func: sbc_nop,
        address_mode: AddressMode::Immediate,
    },
    // 0xEC -
    OpCode {
        name: "CPX",
        func: cpx,
        address_mode: AddressMode::Absolute,
    },
    // 0xED -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::Absolute,
    },
    // 0xEE -
    OpCode {
        name: "INC",
        func: inc,
        address_mode: AddressMode::Absolute,
    },
    // 0xEF -
    OpCode {
        name: "ISC",
        func: isc,
        address_mode: AddressMode::Absolute,
    },
    // 0xF0 -
    OpCode {
        name: "BEQ",
        func: beq,
        address_mode: AddressMode::Offset,
    },
    // 0xF1 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0xF2 -
    OpCode {
        name: "INV",
        func: invalid,
        address_mode: AddressMode::Implied,
    },
    // 0xF3 -
    OpCode {
        name: "ISC",
        func: isc,
        address_mode: AddressMode::IndirectY(false),
    },
    // 0xF4 -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xF5 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xF6 -
    OpCode {
        name: "INC",
        func: inc,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xF7 -
    OpCode {
        name: "ISC",
        func: isc,
        address_mode: AddressMode::ZeroPageIndexedX,
    },
    // 0xF8 -
    OpCode {
        name: "SED",
        func: sed,
        address_mode: AddressMode::Implied,
    },
    // 0xF9 -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0xFA -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::Implied,
    },
    // 0xFB -
    OpCode {
        name: "ISC",
        func: isc,
        address_mode: AddressMode::AbsoluteIndexedY(false),
    },
    // 0xFC -
    OpCode {
        name: "NOP",
        func: nop,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0xFD -
    OpCode {
        name: "SBC",
        func: sbc,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
    // 0xFE -
    OpCode {
        name: "INC",
        func: inc,
        address_mode: AddressMode::AbsoluteIndexedX(true),
    },
    // 0xFF -
    OpCode {
        name: "ISC",
        func: isc,
        address_mode: AddressMode::AbsoluteIndexedX(false),
    },
];

// JuMP
fn jmp(cpu: &mut Cpu, mode: AddressMode) {
    cpu.pc = cpu.operand_address(mode).address();
}

// Jump SubRoutine
fn jsr(cpu: &mut Cpu, mode: AddressMode) {
    let target_address = cpu.operand_address(mode);
    let return_address = cpu.pc - 1;
    cpu.bus.tick();

    cpu.push_stack_16(return_address);
    cpu.pc = target_address.address();
}

/* Load/Store Functions */

fn lda(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);
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

fn inc(cpu: &mut Cpu, mode: AddressMode) {
    do_inc(cpu, mode);
}

fn do_inc(cpu: &mut Cpu, mode: AddressMode) -> u8 {
    let address = cpu.operand_address(mode).address();
    let result = cpu.bus.read(address).wrapping_add(1);
    cpu.bus.tick();
    cpu.bus.write(address, result);

    set_zero_and_negative(cpu, result);

    result
}

fn dec(cpu: &mut Cpu, mode: AddressMode) {
    let address = cpu.operand_address(mode).address();
    let result = cpu.bus.read(address).wrapping_sub(1);
    cpu.bus.tick();
    cpu.bus.write(address, result);
    set_zero_and_negative(cpu, result);
}

/* Stack Instructions */

// PusH Processor status
fn php(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.bus.tick();

    // Bits 4 & 5 are always set on PHP
    //   http://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag
    cpu.push_stack(cpu.p | 0b110000);
}

// PusH Accumulator
fn pha(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.bus.tick();
    cpu.push_stack(cpu.a);
}

// PuLl Processor status
fn plp(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.bus.tick();
    cpu.bus.tick();
    cpu.p = (cpu.pop_stack() & 0b11001111) | (cpu.p & 0b00110000);
}

// PuLl Accumulator
fn pla(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.bus.tick();
    cpu.bus.tick();
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
    cpu.bus.tick();
    cpu.bus.tick();
    let return_address = cpu.pop_stack_16() + 1;
    cpu.pc = return_address;
    cpu.bus.tick();
}

fn rti(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.bus.tick();
    cpu.bus.tick();
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

// INcrement X
fn inx(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.x.wrapping_add(1);
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.x = value;
}

// DEcrement X
fn dex(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.x.wrapping_sub(1);
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.x = value;
}

// INcrement Y
fn iny(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.y.wrapping_add(1);
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.y = value;
}

// DEcrement Y
fn dey(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.y.wrapping_sub(1);
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.y = value;
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
    let value = cpu.a;
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.x = value;
}

// Transfer X to A
fn txa(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.x;
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.a = value;
}

// Transfer A to Y
fn tay(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.a;
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.y = value;
}

// Transfer Y to A
fn tya(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.y;
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.a = value;
}

// Transfer Stack pointer to X
fn tsx(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.sp;
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.x = value;
}

// Transfer X to Stack pointer
fn txs(cpu: &mut Cpu, _mode: AddressMode) {
    let value = cpu.x;
    cpu.bus.tick();
    cpu.sp = value;
}

/* Bit Manipulation Operations */

fn lsr_a(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Carry, cpu.a & 1 != 0);
    cpu.a = cpu.a >> 1;
    set_zero_and_negative(cpu, cpu.a);
    cpu.bus.tick();
}

fn lsr(cpu: &mut Cpu, mode: AddressMode) {
    do_lsr(cpu, mode);
}

fn do_lsr(cpu: &mut Cpu, mode: AddressMode) -> u8 {
    let address = cpu.operand_address(mode).address();
    let operand = cpu.bus.read(address);
    let value = operand >> 1;

    cpu.set_flag(CpuStatus::Carry, operand & 1 != 0);
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.bus.write(address, value);

    value
}

fn asl_a(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Carry, cpu.a & 0x80 > 0);
    cpu.a = cpu.a << 1;
    set_zero_and_negative(cpu, cpu.a);
    cpu.bus.tick();
}

fn asl(cpu: &mut Cpu, mode: AddressMode) {
    do_asl(cpu, mode);
}

fn do_asl(cpu: &mut Cpu, mode: AddressMode) -> u8 {
    let address = cpu.operand_address(mode).address();
    let operand = cpu.bus.read(address);
    let value = operand << 1;

    cpu.set_flag(CpuStatus::Carry, operand & 0x80 != 0);
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.bus.write(address, value);

    value
}

fn ror_a(cpu: &mut Cpu, _mode: AddressMode) {
    let mut value = cpu.a >> 1;

    if cpu.get_flag(CpuStatus::Carry) {
        value |= 0x80;
    }

    cpu.set_flag(CpuStatus::Carry, cpu.a & 1 != 0);
    cpu.a = value;
    set_zero_and_negative(cpu, value);
    cpu.bus.tick();
}

fn ror(cpu: &mut Cpu, mode: AddressMode) {
    do_ror(cpu, mode);
}

fn do_ror(cpu: &mut Cpu, mode: AddressMode) -> u8 {
    let address = cpu.operand_address(mode).address();
    let operand = cpu.bus.read(address);
    let value = (operand >> 1) | (cpu.carry() << 7);

    cpu.set_flag(CpuStatus::Carry, operand & 1 != 0);
    set_zero_and_negative(cpu, value);
    cpu.bus.tick();
    cpu.bus.write(address, value);

    value
}

fn rol_a(cpu: &mut Cpu, _mode: AddressMode) {
    let mut value = cpu.a << 1;

    if cpu.get_flag(CpuStatus::Carry) {
        value += 1;
    }

    cpu.set_flag(CpuStatus::Carry, cpu.a & 0x80 != 0);
    cpu.bus.tick();
    cpu.a = value;
    set_zero_and_negative(cpu, value);
}

fn rol(cpu: &mut Cpu, mode: AddressMode) {
    do_rol(cpu, mode);
}

fn do_rol(cpu: &mut Cpu, mode: AddressMode) -> u8 {
    let address = cpu.operand_address(mode).address();
    let operand = cpu.bus.read(address);
    let value = (operand << 1) | cpu.carry();

    cpu.set_flag(CpuStatus::Carry, operand & 0x80 != 0);
    cpu.bus.tick();
    set_zero_and_negative(cpu, value);
    cpu.bus.write(address, value);

    value
}

/* Miscellaneous Operations */

fn bit(cpu: &mut Cpu, mode: AddressMode) {
    let operand = read_operand(cpu, mode);

    cpu.set_flag(CpuStatus::Zero, operand & cpu.a == 0);
    cpu.set_flag(CpuStatus::Negative, operand & 0b10000000 != 0);
    cpu.set_flag(CpuStatus::Overflow, operand & 0b01000000 != 0);
}

// No OPeration
fn nop(cpu: &mut Cpu, mode: AddressMode) {
    if mode != AddressMode::Implied {
        let _ = read_operand(cpu, mode);
    }
    cpu.bus.tick();
}

// SEt Carry
fn sec(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Carry, true);
    cpu.bus.tick();
}

// CLear Carry
fn clc(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Carry, false);
    cpu.bus.tick();
}

// SEt Interrupt
fn sei(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::InterruptDisable, true);
    cpu.bus.tick();
}

// CLear Interrupt
fn cli(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::InterruptDisable, false);
    cpu.bus.tick();
}

// CLear oVerflow
fn clv(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Overflow, false);
    cpu.bus.tick();
}

// SEt Decimal
fn sed(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Decimal, true);
    cpu.bus.tick();
}

// CLear Decimal
fn cld(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.set_flag(CpuStatus::Decimal, false);
    cpu.bus.tick();
}

fn brk(cpu: &mut Cpu, _mode: AddressMode) {
    cpu.push_stack_16(cpu.pc);
    cpu.push_stack(cpu.p);
    cpu.set_flag(CpuStatus::Break, true);
    cpu.pc = cpu.bus.read_word(0xFFFE);
}

fn dcp(cpu: &mut Cpu, mode: AddressMode) {
    let address = cpu.operand_address(mode).address();
    let result = cpu.bus.read(address).wrapping_sub(1);
    cpu.bus.write(address, result);

    set_zero_and_negative(cpu, result);

    let a = cpu.a;
    set_zero_and_negative(cpu, a.wrapping_sub(result));
    cpu.set_flag(CpuStatus::Carry, a >= result);
}

fn invalid(_cpu: &mut Cpu, _mode: AddressMode) {
    panic!("Invalid operation");
}

// Undocumented operations

fn lax(cpu: &mut Cpu, mode: AddressMode) {
    lda(cpu, mode);
    cpu.x = cpu.a;
}

fn sax(cpu: &mut Cpu, mode: AddressMode) {
    let address = cpu.operand_address(mode).address();
    cpu.bus.write(address, cpu.a & cpu.x);
}

fn sbc_nop(cpu: &mut Cpu, mode: AddressMode) {
    sbc(cpu, mode);
    nop(cpu, AddressMode::Implied);
}

fn isc(cpu: &mut Cpu, mode: AddressMode) {
    let operand = !do_inc(cpu, mode);
    let result = cpu.a as u16 + operand as u16 + cpu.carry() as u16;
    set_carry_and_overflow(cpu, cpu.a, operand, result);
    set_zero_and_negative(cpu, result as u8);
    cpu.a = result as u8;
}

fn slo(cpu: &mut Cpu, mode: AddressMode) {
    let value = cpu.a | do_asl(cpu, mode);

    set_zero_and_negative(cpu, value);
    cpu.a = value;
}

fn rla(cpu: &mut Cpu, mode: AddressMode) {
    let value = do_rol(cpu, mode) & cpu.a;
    set_zero_and_negative(cpu, value);
    cpu.a = value;
}

fn sre(cpu: &mut Cpu, mode: AddressMode) {
    let value = cpu.a ^ do_lsr(cpu, mode);
    set_zero_and_negative(cpu, value);
    cpu.a = value;
}

fn rra(cpu: &mut Cpu, mode: AddressMode) {
    let operand = do_ror(cpu, mode);
    let value = (cpu.a as u16)
        .wrapping_add(operand as u16)
        .wrapping_add(cpu.carry() as u16);

    set_carry_and_overflow(cpu, cpu.a, operand, value);
    set_zero_and_negative(cpu, value as u8);

    cpu.a = value as u8;
}

fn branch(cpu: &mut Cpu, success: bool) {
    // Pull the offset as a signed number
    let offset = read_operand(cpu, AddressMode::Immediate) as i8 as u16;

    if success {
        cpu.bus.tick();
        cpu.pc = cpu.pc.wrapping_add(offset as u16);
    }
}
fn read_operand(cpu: &mut Cpu, mode: AddressMode) -> u8 {
    match cpu.operand_address(mode) {
        AddressModeValue::Absolute(x) => cpu.bus.read(x),
        AddressModeValue::Immediate(x) => x,
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

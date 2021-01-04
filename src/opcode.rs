use super::{address_mode::AddressModeValue, bus::Bus, cpu::Cpu, cpu::CpuStatus};

type OpcodeFunction = fn(&mut Cpu, &mut Bus, AddressModeValue);
#[allow(dead_code)]
pub struct OpCode<'a> {
    name: &'a str,
    func: OpcodeFunction,
    address_mode: AddressModeValue,
    cycles: u8,
}

impl OpCode<'_> {}

#[allow(dead_code)]
pub static OPCODES: [OpCode; 0x100] = [
    // 0x00 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x01 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x02 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x03 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x04 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x05 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x06 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x07 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x08 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x09 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x0A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x0B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x0C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x0D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x0E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x0F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x10 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x11 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x12 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x13 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x14 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x15 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x16 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x17 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x18 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x19 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x1A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x1B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x1C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x1D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x1E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x1F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x20 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x21 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x22 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x23 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x24 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x25 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x26 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x27 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x28 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x29 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x2A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x2B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x2C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x2D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x2E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x2F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x30 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x31 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x32 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x33 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x34 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x35 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x36 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x37 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x38 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x39 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x3A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x3B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x3C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x3D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x3E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x3F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x40 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x41 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x42 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x43 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x44 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x45 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x46 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x47 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x48 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x49 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x4A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x4B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x4C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x4D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x4E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x4F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x50 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x51 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x52 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x53 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x54 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x55 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x56 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x57 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x58 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x59 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x5A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x5B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x5C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x5D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x5E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x5F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x60 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x61 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x62 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x63 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x64 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x65 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x66 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x67 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x68 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x69 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x6A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x6B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x6C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x6D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x6E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x6F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x70 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x71 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x72 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x73 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x74 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x75 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x76 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x77 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x78 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x79 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x7A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x7B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x7C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x7D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x7E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x7F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x80 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x81 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x82 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x83 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x84 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x85 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x86 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x87 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x88 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x89 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x8A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x8B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x8C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x8D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x8E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x8F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x90 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x91 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x92 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x93 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x94 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x95 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x96 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x97 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x98 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x99 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x9A -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x9B -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x9C -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x9D -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x9E -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0x9F -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA0 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA1 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA2 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA3 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA4 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA5 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA6 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA7 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA8 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xA9 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xAA -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xAB -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xAC -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xAD -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xAE -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xAF -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB0 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB1 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB2 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB3 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB4 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB5 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB6 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB7 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB8 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xB9 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xBA -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xBB -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xBC -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xBD -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xBE -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xBF -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC0 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC1 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC2 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC3 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC4 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC5 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC6 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC7 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC8 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xC9 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xCA -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xCB -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xCC -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xCD -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xCE -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xCF -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD0 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD1 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD2 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD3 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD4 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD5 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD6 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD7 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD8 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xD9 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xDA -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xDB -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xDC -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xDD -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xDE -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xDF -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE0 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE1 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE2 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE3 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE4 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE5 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE6 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE7 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xE8 - INX
    OpCode {
        name: "inx",
        func: inx,
        address_mode: AddressModeValue::Implied,
        cycles: 2,
    },
    // 0xE9 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xEA -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xEB -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xEC -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xED -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xEE -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xEF -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF0 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF1 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF2 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF3 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF4 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF5 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF6 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF7 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF8 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xF9 -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xFA -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xFB -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xFC -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xFD -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xFE -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
    // 0xFF -
    OpCode {
        name: "",
        func: nop,
        address_mode: AddressModeValue::Implied,
        cycles: 0,
    },
];

fn nop(_cpu: &mut Cpu, _bus: &mut Bus, _address_mode_value: AddressModeValue) {}

fn inx(cpu: &mut Cpu, _: &mut Bus, mode: AddressModeValue) {
    if let AddressModeValue::Implied = mode {
        cpu.x = cpu.x.wrapping_add(1);
        cpu.set_flag(CpuStatus::Zero, cpu.x == 0);
        cpu.set_flag(CpuStatus::Negative, cpu.x >> 7 == 1);
    } else {
        panic!("INX opcode called with invalid mode: {:#?}", mode);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inx() {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();

        let opcode = &OPCODES[0xE8];

        (opcode.func)(&mut cpu, &mut bus, AddressModeValue::Implied);

        assert_eq!(cpu.x, 1);
        assert!(cpu.get_flag(CpuStatus::Zero) == false);
        assert!(cpu.get_flag(CpuStatus::Negative) == false);
    }
}

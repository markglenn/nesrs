use std::fmt;
#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum AddressModeValue {
    Immediate(u8),
    Relative(u8),
    Absolute(u16),
}

impl AddressModeValue {
    pub fn address(self) -> u16 {
        match self {
            AddressModeValue::Immediate(address) => address as u16,
            AddressModeValue::Absolute(address) => address,
            AddressModeValue::Relative(v) => {
                panic!("Unsupported: AddressModeValue::Relative({})", v)
            }
        }
    }
}
pub enum AddressMode {
    ZeroPage,         // val = PEEK(arg)
    ZeroPageIndexedX, // val = PEEK((arg + X) % 256)
    ZeroPageIndexedY, // val = PEEK((arg + Y) % 256)
    Absolute,         // val = PEEK(arg)
    AbsoluteIndexedX, // val = PEEK(arg + X)
    AbsoluteIndexedY, // val = PEEK(arg + Y)
    IndexedIndirect,  // val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256)
    IndirectIndexed,  // val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y)
    Immediate,        // val = arg
}

impl fmt::Debug for AddressModeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressModeValue::Immediate(address) => write!(f, "Immediate: #{:02x}", address),
            AddressModeValue::Relative(address) => {
                write!(f, "Relative Address: 0x{:02x}", address)
            }
            AddressModeValue::Absolute(address) => {
                write!(f, "Absolute Address: 0x{:04x}", address)
            }
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_variables, unused_mut, unused_imports)]
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(1, 1)
    }
}

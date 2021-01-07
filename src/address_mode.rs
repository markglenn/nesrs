use std::fmt;
#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
pub enum AddressModeValue {
    Implied,
    Relative(u8),
    Absolute(u16),
}

impl fmt::Debug for AddressModeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressModeValue::Implied => write!(f, "Implied"),
            AddressModeValue::Relative(address) => {
                write!(f, "Relative Address: {:02X}", address)
            }
            AddressModeValue::Absolute(address) => {
                write!(f, "Absolute Address: {:04X}", address)
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

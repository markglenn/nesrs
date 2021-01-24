use super::address_register::AddressRegister;
use super::control_register::ControlRegister;
use super::mask_register::MaskRegister;
use super::scroll_register::ScrollRegister;
use super::status_register::StatusRegister;

pub struct PPUState {
    pub ctrl: ControlRegister,  // 0x2000
    pub mask: MaskRegister,     // 0x2001
    pub status: StatusRegister, // 0x2002
    pub scroll: ScrollRegister, // 0x2005
    pub addr: AddressRegister,  // 0x200
}

impl PPUState {
    pub fn new() -> Self {
        PPUState {
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            status: StatusRegister::new(),
            scroll: ScrollRegister::new(),
            addr: AddressRegister::new(),
        }
    }
}

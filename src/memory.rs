pub struct Memory {
    block: [u8; 0x10000], // 64K memory block
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            block: [0; 0x10000],
        }
    }

    /*
    pub fn read_u8(&self, arg: u8, cpu: &Cpu, mode: AddressingMode) -> u8 {
        match mode {
            AddressingMode::ZeroPageIndexX => self.zero_page_index_read(arg, cpu.x),
            AddressingMode::ZeroPageIndexY => self.zero_page_index_read(arg, cpu.y),
            AddressingMode::AbsoluteIndexedX => self.absolute_index_read(arg, cpu.x),
            AddressingMode::AbsoluteIndexedY => self.absolute_index_read(arg, cpu.y),
            AddressingMode::IndexedIndirect => self.index_indirect_read(arg, cpu.x),
            AddressingMode::IndirectIndexed => self.indirect_index_read(arg, cpu.y),
        }
    }

    fn zero_page_index_read(&self, arg: u8, register: u8) -> u8 {
        self.block[(arg as usize + register as usize) % 256]
    }

    fn absolute_index_read(&self, arg: u8, register: u8) -> u8 {
        self.block[arg as usize + register as usize]
    }

    fn index_indirect_read(&self, arg: u8, register: u8) -> u8 {
        let offset = arg as usize + register as usize;

        let low_byte = self.block[offset % 256] as usize;
        let high_byte = self.block[(offset + 1) % 256] as usize;
        self.block[low_byte | high_byte << 8]
    }

    fn indirect_index_read(&self, arg: u8, register: u8) -> u8 {
        let low_byte = self.block[arg as usize] as usize;
        let high_byte = self.block[(arg as usize + 1) % 256] as usize;

        self.block[low_byte | high_byte << 8 + register as usize]
    }
    */
}

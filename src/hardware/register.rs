#[derive(Debug, Clone, Copy)]
pub struct Register<T> {
    data: T,
}

impl Register<u8> {
    pub fn new() -> Register<u8> {
        Register { data: 0 }
    }

    pub fn new_with_value(value: u8) -> Register<u8> {
        Register { data: value }
    }

    pub fn load(&self) -> u8 {
        self.data
    }

    pub fn store(&mut self, value: u8) {
        self.data = value;
    }

    pub fn store_bit(&mut self, pos: u8, value: bool) {
        let numeric_value: u8 = if value { 1 } else { 0 };
        self.data = self.data & !(1 << pos) | ((numeric_value & 1) << pos);
    }

    pub fn load_bit(&self, pos: u8) -> u8 {
        (self.data >> pos) & 1
    }

    pub fn is_set(&self, pos: u8) -> bool {
        self.load_bit(pos) == 1
    }

    pub fn clear(&mut self) {
        self.data = 0;
    }

    pub fn set_bit(&mut self, pos: u8) {
        self.store_bit(pos, true);
    }

    pub fn clear_bit(&mut self, pos: u8) {
        self.store_bit(pos, false);
    }

    pub fn increment(&mut self) {
        self.add(1);
    }

    pub fn decrement(&mut self) {
        self.subtract(1);
    }

    pub fn shift(&mut self, value: u8) -> u8 {
        let carry = self.load_bit(7);
        self.data = (self.data << 1) | (value & 1);
        carry
    }

    fn add(&mut self, value: u8) {
        self.data = self.data.wrapping_add(value);
    }

    fn subtract(&mut self, value: u8) {
        self.data = self.data.wrapping_sub(value);
    }
}

impl Register<u16> {
    pub fn new() -> Register<u16> {
        Register { data: 0 }
    }

    pub fn load(&self) -> u16 {
        self.data
    }

    pub fn store(&mut self, value: u16) {
        self.data = value;
    }

    pub fn store_bit(&mut self, pos: u8, value: bool) {
        let numeric_value: u16 = if value { 1 } else { 0 };
        self.data = self.data & !(1 << pos) | ((numeric_value & 1) << pos);
    }

    pub fn load_bit(&self, pos: u8) -> u8 {
        ((self.data >> pos) & 1) as u8
    }

    pub fn is_set(&self, pos: u8) -> bool {
        self.load_bit(pos) == 1
    }

    pub fn clear(&mut self) {
        self.data = 0;
    }

    pub fn set_bit(&mut self, pos: u8) {
        self.store_bit(pos, true);
    }

    pub fn clear_bit(&mut self, pos: u8) {
        self.store_bit(pos, false);
    }

    pub fn increment(&mut self) {
        self.add(1);
    }

    pub fn decrement(&mut self) {
        self.subtract(1);
    }

    pub fn shift(&mut self, value: u8) -> u8 {
        let carry = self.load_bit(15);
        self.data = (self.data << 1) | (value as u16 & 1);
        carry
    }

    fn add(&mut self, value: u16) {
        self.data = self.data.wrapping_add(value);
    }

    fn subtract(&mut self, value: u16) {
        self.data = self.data.wrapping_sub(value);
    }
}

#[cfg(test)]
mod tests_register_u8 {
    use super::*;

    #[test]
    fn initial_value() {
        let r = Register::<u8>::new();
        assert_eq!(0, r.data);
    }

    #[test]
    fn load() {
        let r = Register::<u8>::new();
        assert_eq!(0, r.load());
    }

    #[test]
    fn load_bit() {
        let mut r = Register::<u8>::new();
        r.store(2);
        assert_eq!(1, r.load_bit(1));
    }

    #[test]
    fn clear() {
        let mut r = Register::<u8>::new();
        r.store(5);
        r.clear();

        assert_eq!(0, r.data);
    }

    #[test]
    fn is_set() {
        let mut r = Register::<u8>::new();
        assert_eq!(false, r.is_set(1));
        r.set_bit(1);
        assert_eq!(true, r.is_set(1));
    }

    #[test]
    fn set_bit() {
        let mut r = Register::<u8>::new();
        assert_eq!(false, r.is_set(1));
        r.set_bit(1);
        assert_eq!(true, r.is_set(1));
    }

    #[test]
    fn clear_bit() {
        let mut r = Register::<u8>::new();
        r.set_bit(1);
        assert_eq!(true, r.is_set(1));
        r.clear_bit(1);
        assert_eq!(false, r.is_set(1));
    }

    #[test]
    fn shift() {
        let mut r = Register::<u8>::new();
        assert_eq!(0, r.shift(1));
        assert_eq!(1, r.load());
        assert_eq!(0, r.shift(0));
        assert_eq!(2, r.load());
        r.store(0xFF);
        assert_eq!(1, r.shift(1));
        assert_eq!(0xFF, r.load());
        assert_eq!(1, r.shift(0));
        assert_eq!(0xFE, r.load());
    }

    #[test]
    fn increment() {
        let mut r = Register::<u8>::new();
        r.increment();

        assert_eq!(1, r.load());

        r.store(0xFF);
        r.increment();
        assert_eq!(0, r.load());
    }

    #[test]
    fn decrement() {
        let mut r = Register::<u8>::new();
        r.decrement();
        assert_eq!(0xFF, r.load());
        r.decrement();
        assert_eq!(0xFE, r.load());
    }
}

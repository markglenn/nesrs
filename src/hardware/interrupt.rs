pub struct Interrupt {
    schedule: Option<u8>,
}

impl Interrupt {
    pub fn new() -> Self {
        Interrupt { schedule: None }
    }

    pub fn tick(&mut self) {
        match self.schedule.as_mut() {
            Some(0) | None => (),
            Some(t) => *t -= 1,
        }
    }

    pub fn schedule(&mut self, ticks: u8) {
        self.schedule = Some(ticks);
    }

    pub fn acknowledge(&mut self) {
        self.schedule = None;
    }

    pub fn ready(&self) -> bool {
        self.schedule == Some(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn schedule_an_interrupt() {
        let mut interrupt = Interrupt::new();
        assert!(!interrupt.ready());

        interrupt.schedule(1);
        assert!(!interrupt.ready());
        interrupt.tick();
        assert!(interrupt.ready());

        interrupt.acknowledge();
        assert!(!interrupt.ready());
    }
}

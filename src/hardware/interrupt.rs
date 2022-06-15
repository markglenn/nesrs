pub struct Interrupt {
    schedule: Option<usize>,
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

    pub fn schedule(&mut self, ticks: usize) {
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
        assert_eq!(false, interrupt.ready());

        interrupt.schedule(1);
        assert_eq!(false, interrupt.ready());
        interrupt.tick();
        assert_eq!(true, interrupt.ready());

        interrupt.acknowledge();
        assert_eq!(false, interrupt.ready());
    }
}

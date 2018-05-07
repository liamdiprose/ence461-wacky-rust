use mat91lib as mt91;

pub type Duration = u32;

pub struct Pacer {
    period: u32
}

impl Pacer {

    pub fn new (rate: u32) -> Self {
        let period = super::F_CPU / rate;

        unsafe {
            mt91::pit_init();
            Pacer {
                period: period
            }
        }

    }

    pub fn wait(&mut self) {
        unsafe {
            mt91::pit_wait(self.period);
        }
    }
}
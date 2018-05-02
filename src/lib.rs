#![no_std]
extern crate mat91lib;



mod pwm;
mod spi;
mod twi;
mod pio;


pub use pwm::Pwm;
pub use spi::Spi;
pub use twi::Twi;

pub use pio::Pio;

pub mod peripherals {
    const TIMER1: u32 = 2;
}

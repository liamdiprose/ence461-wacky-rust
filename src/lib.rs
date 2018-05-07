#![no_std]
extern crate mat91lib;

pub const F_CPU: u32 = 96_000_000;

mod pwm;
mod spi;
mod twi;
pub mod pio;
mod timer;
mod pit;


pub use pwm::Pwm;
pub use spi::Spi;
pub use twi::Twi;
pub use pio::Pio;
pub use timer::Timer;
pub use pit::Pacer;


pub mod peripherals {

    use super::mat91lib as mt91;

    enum Timer {
        T0, // = super::Timer { channel: mt91::TIMER_CHANNEL_0 as u8 },
        T1, // = super::Timer { channel: mt91::TIMER_CHANNEL_1 as u8 },
        T2, // = super::Timer { channel: mt91::TIMER_CHANNEL_2 as u8 }
    }

    pub use super::Pio;

    pub enum Pwm {
    }

    pub enum Twi {


    }

}


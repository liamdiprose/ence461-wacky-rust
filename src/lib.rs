#![no_std]
extern crate mat91lib;



mod pwm;

pub use pwm::Pwm;

fn main() {
    let buzzer = Pwm::new(1, 1, 1, true).unwrap();

    buzzer.start();
}

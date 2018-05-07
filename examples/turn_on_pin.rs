#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate mat91lib;
extern crate wacky;

use wacky::{Pio, Pwm};

use ::core::fmt::Arguments;


pub fn main() {
    let led = Pio::A0;
    led.set_output(wacky::pio::LogicLevel::High).unwrap();
}

type PanicLocation = (&'static str, u32);

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_fmt: Arguments, _: &PanicLocation) -> ! {
    // TODO: Serial Error Message
    loop {}
}

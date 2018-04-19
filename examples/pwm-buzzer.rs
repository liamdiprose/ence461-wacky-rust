#![no_std]
#![feature(lang_items)]
#![no_main]

extern crate wacky;

use ::core::fmt::Arguments;

#[no_mangle]
pub fn main () {
    let buzzer = wacky::Pwm::new(1, 1, 1, true).unwrap();
    buzzer.set_frequency(4000);
    buzzer.start();
}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_fmt: Arguments, _:&(&'static str, u32)) -> ! {
    // TODO: Serial Error Message
    loop {}
}

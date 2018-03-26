extern crate wacky;

fn main () {
    let buzzer = wacky::Pwm::new(1, 1, 1, true).unwrap();
    buzzer.set_frequency(4000);
    buzzer.start();
}

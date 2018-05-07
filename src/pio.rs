// TODO: PIO Enums
#![allow(non_snake_case)]

use mat91lib as mt91;

use core::ops;
#[derive(Copy, Clone)]
pub enum LogicLevel {
    High,
    Low
}

impl ops::Not for LogicLevel {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            LogicLevel::High => LogicLevel::Low,
            LogicLevel::Low => LogicLevel::High
        }
    }
}

#[repr(u32)]
#[derive(Copy,Clone)]
pub enum Pio {
    A0 = {(0 << 5) + 0},
    A1 = {(0 << 5) + 1},
    A2 = {(0 << 5) + 2},
    A3 = {(0 << 5) + 3},
    A4 = {(0 << 5) + 4},
    A5 = {(0 << 5) + 5},
    A6 = {(0 << 5) + 6},
}

impl Pio {
    pub fn init(&self) {

        let reg = *self as u32;

        unsafe {
            mt91::pio_init(reg);
        }
    }

    pub fn set_output(&self, level: LogicLevel) -> Result<(),()> {
        unsafe {
            match level {
                LogicLevel::High =>
                    match mt91::pio_config_set(self.pointer(), mt91::pio_config_enum_PIO_OUTPUT_HIGH) {
                        1 => Ok(()),
                        _ => Err(())
                    }
                LogicLevel::Low =>
                    match mt91::pio_config_set(self.pointer(), mt91::pio_config_enum_PIO_OUTPUT_LOW) {
                        1 => Ok(()),
                        _ => Err(())
                    }
            }
        }
    }

    pub fn turn_level(&self, level: LogicLevel) -> Result<(),()> {
        unsafe {
            match level {
                LogicLevel::Low => self.turn_low(),
                LogicLevel::High => self.turn_high()
            }
        }
    }

    pub fn turn_low(&self) -> Result<(),()> {
    unsafe {
        match mt91::pio_output_low(self.pointer()) {
            () => Ok(()),
            _ => Err(())
        }
    }
    }

    pub fn turn_high(&self) -> Result<(), ()> {
        unsafe {
            match mt91::pio_output_high(self.pointer()) {
                () => Ok(()),
                _ => Err(())
            }
        }
    }

    fn pointer(&self) -> u32 {
        *self as u32
    }
}

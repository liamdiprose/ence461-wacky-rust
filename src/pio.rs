// TODO: PIO Enums
#![allow(non_snake_case)]

use mat91lib as mt91;

#[repr(u32)]
pub enum Pio {
    A0 = mt91::PIO_AIMDR_P0
}


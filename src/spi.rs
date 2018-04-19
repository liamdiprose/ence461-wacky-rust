use mat91lib as mt91;
use ::Pio;
use ::core::mem::size_of_val;

pub struct Spi {
    spi: mt91::spi_t
}

impl Spi {
    pub fn new (cs_pio: Pio, mode: mt91::spi_mode_t, clock_speed_khz: u32, bits: u8) -> Result<Self, ()> {
        let cfg = mt91::spi_cfg_t {
            channel: 0,
            clock_speed_kHz: clock_speed_khz,
            cs: cs_pio as u32,
            mode,
            cs_mode: mode,
            bits
        };

        unsafe {
            let me = Spi {
                spi: mt91::spi_init(&cfg)
            };
            Ok(me)
        }
    }

    pub fn transact (&self, buff: &mut mt91::spi_transfer_t) {
        unsafe {
            mt91::spi_transact(self.spi, buff, size_of_val(buff) as u8);
        }
    }

    pub fn transfer (&self, tx: &mut usize, rx: &mut usize, terminate: bool) {
        unsafe {
            mt91::spi_transfer(self.spi, tx, rx, size_of_val(tx) as u16, terminate as u8);
        }
    }

    pub fn shutdown (&self) {
        unsafe {
            mt91::spi_shutdown(self.spi)
        }
    }
}


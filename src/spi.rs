use mat91lib as mt91;

struct Spi {
    spi: mt91::spi_t
}

impl Spi {
    pub fn new (cs_pio: Pio, mode: spi_mode, clock_speed_khz: usize, bits: u8) -> Result<(), ()> {
        let mut cfg = spi_cfg_t {
            channel: 0,
            clock_speed_kHz: clock_speed_khz,
            cs: cs_pio,
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

    pub fn transact (&self, buff: &[u8]) {
        mt91::spi_transact(self.spi, &buff, buff.len())
    }

    pub fn transfer (&self, tx: &[u8], rx: &[u8], terminate: bool) {
        mt91::spi_transfer(self.spi, tx, rx, tx.len(), terminate)
    }

    pub fn shutdown (&self) {
        mat91::spi_shutdown(self.spi)
    }
}


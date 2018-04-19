use mat91lib as mt91;

pub struct Twi {
    twi: mt91::twi_t
}

pub type Address = usize;

impl Twi {
    pub fn new (channel: u32, clock_speed: u16) -> Result<Self, ()> {
        let mut cfg = twi_cfg {
            channel,
            period: clock_speed // TODO: TWI_PERIOD_DIVISOR
        };

        unsafe {
            let me = Twi {
                twi: twi_init(&cfg)
            };

            Ok(me)
        }
    }

    pub fn write (&self, slave: Address, iaddr: Address, buffer: &[u8]) {
        mt91::twi_master_addr_write_timeout(self.twi, slave, iaddr, iaddr.len(), buffer, buffer.len())
    }

    pub fn read (&self, slave: Address, iaddr: Address, buffer: &[u8]) {
        mt91::twi_master_addr_read_timeout(self.twi, slave, buffer, buffer.len(), 100);  // TODO: Read timeout
    }

    // TODO: read timeout

    pub fn shutdown(&self) {
        mt91::twi_shutdown(self.twi)
    }
}

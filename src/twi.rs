use mat91lib as mt91;

use ::core::mem::size_of_val;

pub struct Twi {
    twi: mt91::twi_t
}

pub type Address = usize;

impl Twi {
    pub fn new (channel: u32, clock_speed: u16) -> Result<Self, ()> {
        let cfg = mt91::twi_cfg_t {
            channel: channel as u8,
            period: clock_speed,// TODO: TWI_PERIOD_DIVISOR
            slave_addr: 0
        };

        unsafe {
            let me = Twi {
                twi: mt91::twi_init(&cfg)
            };

            Ok(me)
        }
    }

    pub fn write (&self, slave: Address, iaddr: Address, buffer: &usize) {
        unsafe {
            mt91::twi_master_addr_write_timeout(self.twi,
                                                slave as u8,
                                                iaddr as u32,
                                                size_of_val(&iaddr) as u8,
                                                buffer,
                                                size_of_val(buffer) as u16,
                                                1000);
        }
    }

    pub fn read (&self, slave: Address, iaddr: Address, buffer: &mut usize) {
        unsafe {
            mt91::twi_master_addr_read_timeout(self.twi,
                                               slave as u8,
                                               iaddr as u32,
                                               size_of_val(&iaddr) as u8,
                                               buffer,
                                               size_of_val(buffer) as u16,
                                               1000);  // TODO: Read timeout
        }
    }

    // TODO: read timeout

    pub fn shutdown(&self) {
        unsafe {
            mt91::twi_shutdown(self.twi)
        }
    }

}

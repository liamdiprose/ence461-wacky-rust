use mat91lib as mt91;

use ::core::mem::size_of_val;

pub struct Twi {
    twi: mt91::twi_t
}

pub type Address = u32;

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

    pub fn write (&self, slave: Address, iaddr: Address, buffer: &mut [u8]) {
        unsafe {
            mt91::twi_master_addr_write_timeout(self.twi,
                                                slave as u8,
                                                iaddr as u32,
                                                size_of_val(&iaddr) as u8,
                                                buffer.as_mut_ptr() as *mut usize,
                                                size_of_val(buffer) as u16,
                                                1000);
        }
    }


    pub fn read (&self, slave: Address, iaddr: Address, buffer: &mut [u8]) {
        unsafe {
            mt91::twi_master_addr_read_timeout(self.twi,
                                               slave as u8,
                                               iaddr as u32,
                                               size_of_val(&iaddr) as u8,
                                               buffer.as_mut_ptr() as *mut usize,
                                               size_of_val(buffer) as u16,
                                               1000);  // TODO: Read timeout
        }
    }

    pub fn read_byte(&self, slave: Address, iaddr: Address) -> u8 {
        let mut buffer = [0u8];  // Single byte
        self.read(slave, iaddr, &mut buffer);

        return buffer[0];
    }

    pub fn shutdown(&self) {
        unsafe {
            mt91::twi_shutdown(self.twi)
        }
    }

}

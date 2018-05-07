use mat91lib as mt91;

pub struct Timer {
    channel: u8
}

impl Timer {

    pub fn new(channel: u8) -> Self {
        unsafe {
            mt91::timer_init(channel);
        }
        Timer {
            channel
        }
    }

    pub fn enable(self) {
        unsafe {
            mt91::timer_clock_enable(self.channel);
        }
    }

    pub fn disable(self) {
        unsafe {
            mt91::timer_disable(self.channel);
        }
    }

    pub fn set_prescaler(self, period: u32) -> Result<(),()> {
        unsafe {
            match mt91::timer_prescaler_set(self.channel, period) {
                0 => Ok(()),
                _ => Err(())
            }
        }
    }

    // Configure the timer to run in capture mode
    pub fn set_capture_mode(self) -> Result<(),()> {
        unsafe {
            match mt91::timer_capture_mode_set(self.channel) {
                0 => Ok(()),
                _ => Err(())
            }
        }
    }

    pub fn configure_trigger_mode(self, trigger_edge: u8, external_trigger: u8, compare_trigger: u8) -> Result<(),()> {

        unsafe {
            match mt91::timer_capture_trigger_config(self.channel,
                                                     trigger_edge,
                                                     external_trigger,
                                                     compare_trigger) {
                0 => Ok(()),
                _ => Err(())
            }
        }
    }

    pub fn capture_register_load_config(&self, load_ra: u8, load_rb: u8, load_rb_clk: u8) -> Result<(),()> {
        unsafe {
            match mt91::timer_capture_register_load_config(self.channel, load_ra, load_rb, load_rb_clk) {
                1 => Ok(()),
                _ => Err(())
            }
        }
    }

    pub fn clock_enable(&self) -> Result<(),()> {
        unsafe {
            match mt91::timer_clock_enable(self.channel) {
                0 => Ok(()),
                _ => Err(())
            }
        }
    }

    pub fn read_current_value(&self) -> u16 {
        unsafe {
            mt91::timer_read(self.channel, mt91::TIMER_TC_CV as u8)
        }
    }

    pub fn read_ra(&self) -> u16 {
        unsafe {
            mt91::timer_read(self.channel, mt91::TIMER_TC_RA as u8)
        }
    }

    pub fn read_rb(&self) -> u16 {
        unsafe {
            mt91::timer_read(self.channel, mt91::TIMER_TC_RB as u8)
        }
    }

    pub fn read_rc(&self) -> u16 {
        unsafe {
            mt91::timer_read(self.channel, mt91::TIMER_TC_RC as u8)
        }
    }

    pub fn reset(&self) -> Result<(),()> {
        unsafe {
            match mt91::timer_reset(self.channel) {
                0 => Ok(()),
                _ => Err(())
            }
        }
    }
}


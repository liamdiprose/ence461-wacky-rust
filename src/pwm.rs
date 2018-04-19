
use mat91lib as mt91;
use ::Pio;

pub struct Pwm {
    pwm: mt91::pwm_t
}

impl Pwm {
    pub fn new(pio: Pio, frequency: u32, duty: u32, invert: bool) -> Result<Self, ()> {
        let mut cfg = mt91::pwm_cfg_t {
            pio: pio as u32,
            period: frequency,
            duty: 0,
            align: mt91::pwm_align_t_PWM_ALIGN_LEFT,
            polarity: if invert { mt91::pwm_polarity_t_PWM_POLARITY_LOW } else { mt91::pwm_polarity_t_PWM_POLARITY_HIGH },
            duty_ppt: 0,
            frequency: 0,
            stop_state: 0
        };


        unsafe {
            let pwm = Pwm { pwm: mt91::pwm_init(&mut cfg) };

            // TODO: Check if successful (see https://eng-git.canterbury.ac.nz/mph/wacky-racers-2018/blob/master/src/wackylib/Pwm.cpp#L17)

            pwm.set_duty(duty);
            pwm.start();

            Ok(pwm)
        }
    }

    pub fn set_duty(&self, duty: u32) {
        unsafe {
            mt91::pwm_duty_ppt_set(self.pwm, duty.into());
        }
    }

    pub fn set_frequency(&self, frequency: u32) {
        unsafe {
            mt91::pwm_frequency_set(self.pwm, frequency.into());
        }
    }

    pub fn start (&self) {
        unsafe {
            mt91::pwm_start(self.pwm)
        }
    }

    pub fn stop (&self) {
        unsafe {
            mt91::pwm_stop(self.pwm)
        }
    }
}

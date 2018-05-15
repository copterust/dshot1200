extern crate stm32f103xx_hal as hal;
use hal::pwm::{ C4 };
use stm32f103xx::{ TIM4 };
use hal::prelude::*;

pub struct DShotESC {
    frame: u16,
    pwm: hal::pwm::Pwm<TIM4, C4>
}

impl DShotESC {
    pub fn new(mut pwm: hal::pwm::Pwm<TIM4, C4>) -> Self {
        pwm.enable();
        DShotESC { frame: 0, pwm: pwm }
    }

    pub fn reset(&mut self) {
        for _ in 0..16 {
            self.pwm.set_duty(0);
        }
    }

    pub fn set_value(&mut self, value: u16, request_telemetry: bool) {
        self.frame = value << 1;
        self.frame |= if request_telemetry { 1 } else { 0 };

        let mut check_sum = 0;
        let mut check_sum_data = value;
        for _ in 0..2 {
            check_sum ^= check_sum_data;
            check_sum_data >>= 4;
        }
        check_sum &= 0x0f;
        self.frame = (self.frame << 4) | check_sum;
        let max = self.pwm.get_max_duty();
        let mut to_send = self.frame;
        for _ in 0..16 {
            if to_send & 0x8000 == 0 {
                self.pwm.set_duty(1 * max / 3);
            } else {
                self.pwm.set_duty(2 * max / 3);
            }
            to_send <<= 1;
        }
    }
}
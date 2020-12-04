use crate::clock::Clocks;
use embedded_hal::blocking::delay::{DelayUs, DelayMs};
use litex_pac::peripherals::timer0;
use litex_pac::{read_reg, write_reg};

pub struct Delay {
    timer: timer0::Instance,
    freq: u32,
}

impl Delay {
    pub fn new(timer: timer0::Instance, clocks: &Clocks) -> Self {
        Delay {
            timer,
            freq: clocks.sysclk().0,
        }
    }

    fn _delay_us(&mut self, us: u64) {
        let ticks = ((self.freq as u64) * us) / 1_000_000;
        write_reg!(timer0, self.timer, EN, en: 0);
        write_reg!(timer0, self.timer, RELOAD, reload: 0);
        write_reg!(timer0, self.timer, LOAD, load: ticks as u32);
        write_reg!(timer0, self.timer, EN, en: 1);

        loop {
            write_reg!(timer0, self.timer, UPDATE_VALUE, update_value: 1);
            if read_reg!(timer0, self.timer, VALUE, value) == 0 {
                break;
            }
        }
    }
}

impl DelayUs<u32> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: u32) {
        self._delay_us(us as u64)
    }
}

// Implemented for constructions like `delay.delay_us(50_000);`
impl DelayUs<i32> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: i32) {
        assert!(us >= 0);
        self.delay_us(us as u32);
    }
}

impl DelayUs<u16> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: u16) {
        self.delay_us(us as u32)
    }
}

impl DelayUs<u8> for Delay {
    #[inline(always)]
    fn delay_us(&mut self, us: u8) {
        self.delay_us(us as u32)
    }
}

impl DelayMs<u32> for Delay {
    fn delay_ms(&mut self, ms: u32) {
        self._delay_us((ms as u64) * 1000)
    }
}

// Implemented for constructions like `delay.delay_ms(50_000);`
impl DelayMs<i32> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: i32) {
        assert!(ms >= 0);
        self.delay_ms(ms as u32);
    }
}

impl DelayMs<u16> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(ms as u32)
    }
}

impl DelayMs<u8> for Delay {
    #[inline(always)]
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(ms as u32)
    }
}
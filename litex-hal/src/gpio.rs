use litex_pac::peripherals::gpio;
use litex_pac::{read_reg, write_reg, modify_reg};
use core::marker::PhantomData;
use riscv::interrupt;
use embedded_hal::digital::v2::*;
use core::convert::Infallible;

pub trait GpioExt {
    fn split(self) -> Pins;
}

impl GpioExt for gpio::Instance {
    fn split(self) -> Pins {
        Pins {
            taken_mask: 0
        }
    }
}


pub struct Pins {
    taken_mask: u32,
}

impl Pins {
    pub fn pin(&mut self, index: u8) -> Option<Pin<Input>> {
        let mask = 1u32 << index;
        if self.taken_mask & mask == 0 {
            self.taken_mask |= mask;

            Some(Pin {
                index,
                _mode: PhantomData,
            })
        } else {
            None
        }
    }
}

/// Input mode (type state)
pub struct Input;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push-pull output mode (type state)
pub struct PushPull;

/// Open-drain output mode (type state)
pub struct OpenDrain;

pub struct Pin<MODE> {
    index: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE> Pin<MODE> {
    pub fn into_input(self) -> Pin<Input> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        interrupt::free(|_| {
            modify_reg!(gpio, gpio, DIR, |v| v & !mask);
        });

        Pin {
            index: self.index,
            _mode: PhantomData,
        }
    }

    pub fn into_push_pull_output(self) -> Pin<Output<PushPull>> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        interrupt::free(|_| {
            modify_reg!(gpio, gpio, DIR, |v| v | mask);
            modify_reg!(gpio, gpio, OTYPER, |v| v & !mask)
        });

        Pin {
            index: self.index,
            _mode: PhantomData,
        }
    }

    pub fn into_open_drain_output(self) -> Pin<Output<OpenDrain>> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        interrupt::free(|_| {
            modify_reg!(gpio, gpio, DIR, |v| v | mask);
            modify_reg!(gpio, gpio, OTYPER, |v| v | mask);
        });

        Pin {
            index: self.index,
            _mode: PhantomData,
        }
    }
}

impl InputPin for Pin<Input> {
    type Error = Infallible;

    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        Ok(read_reg!(gpio, gpio, IDR) & mask != 0)
    }

    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        Ok(read_reg!(gpio, gpio, IDR) & mask == 0)
    }
}

impl<MODE> OutputPin for Pin<Output<MODE>> {
    type Error = Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        write_reg!(gpio, gpio, BRR, mask);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        write_reg!(gpio, gpio, BSR, mask);
        Ok(())
    }
}

impl<MODE> StatefulOutputPin for Pin<Output<MODE>> {
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        Ok(read_reg!(gpio, gpio, ODR) & mask != 0)
    }

    fn is_set_low(&self) -> Result<bool, Self::Error> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        Ok(read_reg!(gpio, gpio, ODR) & mask == 0)
    }
}

impl<MODE> ToggleableOutputPin for Pin<Output<MODE>> {
    type Error = Infallible;

    fn toggle(&mut self) -> Result<(), Self::Error> {
        let gpio = unsafe { &*litex_pac::gpio::GPIO };
        let mask = 1u32 << self.index;
        if read_reg!(gpio, gpio, ODR) & mask != 0 {
            self.set_low()
        } else {
            self.set_high()
        }
    }
}
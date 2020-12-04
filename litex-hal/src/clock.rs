use crate::time::Hertz;

pub struct Clocks {
    sysclk: u32
}

impl Clocks {
    pub fn new() -> Clocks {
        let sysclk = match () {
            #[cfg(with_clock)]
            () =>  {
                use litex_pac::clock;
                use litex_pac::read_reg;
                let clock = unsafe { clock::CLOCK::conjure() };
                read_reg!(clock, clock, CORECLK)
            },
            #[cfg(not(with_clock))]
            () => 21_000_000,
        };
        Clocks {
            sysclk
        }
    }

    #[inline]
    pub fn sysclk(&self) -> Hertz {
        Hertz(self.sysclk)
    }
}

use crate::clock::Clocks;
use crate::time::Bps;
use core::convert::Infallible;
use embedded_hal::serial;
use litex_pac::peripherals::{uart, uart_phy};
use litex_pac::{read_reg, write_reg};

pub struct Serial {
    uart: uart::Instance,
    phy: uart_phy::Instance,
}

impl Serial {
    pub fn new(
        uart: uart::Instance,
        phy: uart_phy::Instance,
        baud_rate: Bps,
        clocks: &Clocks,
    ) -> Self {
        let tune = (((baud_rate.0 as u64) << 32) / (clocks.sysclk().0 as u64)) as u32;
        write_reg!(uart_phy, phy, TUNING_WORD, tune);

        Self { uart, phy }
    }

    pub fn free(self) -> (uart::Instance, uart_phy::Instance) {
        (self.uart, self.phy)
    }
}

impl serial::Read<u8> for Serial {
    type Error = Infallible;

    fn read(&mut self) -> nb::Result<u8, Infallible> {
        if read_reg!(uart, self.uart, RXEMPTY, rxempty) != 0 {
            Err(nb::Error::WouldBlock)
        } else {
            let byte = read_reg!(uart, self.uart, RXTX, rxtx) as u8;
            Ok(byte)
        }
    }
}

impl serial::Write<u8> for Serial {
    type Error = Infallible;

    fn write(&mut self, byte: u8) -> nb::Result<(), Infallible> {
        if read_reg!(uart, self.uart, TXFULL, txfull) != 0 {
            Err(nb::Error::WouldBlock)
        } else {
            write_reg!(uart, self.uart, RXTX, rxtx: byte as u32);
            Ok(())
        }
    }

    fn flush(&mut self) -> nb::Result<(), Infallible> {
        if read_reg!(uart, self.uart, TXEMPTY, txempty) == 0 {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl embedded_hal::blocking::serial::write::Default<u8> for Serial {}

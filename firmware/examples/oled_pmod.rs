#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use litex_hal::prelude::*;
use litex_hal::pac;
use litex_hal::clock::Clocks;
use litex_hal::println;
use litex_hal::serial::Serial;
use litex_hal::hal::timer::{CountDown, Periodic};
use bitbang_hal::spi::{SPI, MODE_0};
use core::convert::Infallible;
use ssd1331::{Ssd1331, DisplayRotation};
use litex_hal::delay::Delay;
use embedded_graphics::prelude::*;
use embedded_graphics::image::{ImageRawLE, Image};


struct SpiClockTimer;

impl CountDown for SpiClockTimer {
    type Time = ();

    fn start<T>(&mut self, _count: T)
        where T: Into<Self::Time>
    {
    }

    #[inline(always)]
    fn wait(&mut self) -> nb::Result<(), void::Void> {
        // The display is fast, don't need to wait
        Ok(())
    }
}

impl Periodic for SpiClockTimer {}

struct FakeMisoPin;

impl litex_hal::hal::digital::v2::InputPin for FakeMisoPin {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }
}


#[entry]
fn main() -> ! {
    let clocks = Clocks::new();

    let uart = unsafe { pac::uart::UART::conjure() };
    let uart_phy = unsafe { pac::uart_phy::UART_PHY::conjure() };
    let serial = Serial::new(uart, uart_phy, 115200.bps(), &clocks);
    litex_hal::stdout::set_serial(serial);

    println!("Firmware starting...");
    println!("Core frequency: {} MHz", clocks.sysclk().0 / 1_000_000);

    let timer = pac::timer0::TIMER0::take().unwrap();
    let mut delay = Delay::new(timer, &clocks);

    let gpio = unsafe { pac::gpio::GPIO::conjure() };
    let mut pins = gpio.split();

    let pmod_offset = 16; // PMOD2
    let mut cs = pins.pin(pmod_offset).unwrap().into_push_pull_output();
    let mosi = pins.pin(pmod_offset+1).unwrap().into_push_pull_output();
    let sck = pins.pin(pmod_offset+3).unwrap().into_push_pull_output();
    let dc = pins.pin(pmod_offset+4).unwrap().into_push_pull_output();
    let mut rst = pins.pin(pmod_offset+5).unwrap().into_push_pull_output();
    let mut en = pins.pin(pmod_offset+6).unwrap().into_push_pull_output();

    cs.set_low().ok();
    rst.set_high().ok();
    en.set_high().ok();

    let spi = SPI::new(MODE_0, FakeMisoPin, mosi, sck, SpiClockTimer);

    let mut led_red = pins.pin(24).unwrap().into_push_pull_output();
    let mut led_green = pins.pin(25).unwrap().into_push_pull_output();
    led_red.set_high().ok();

    let mut disp = Ssd1331::new(spi, dc, DisplayRotation::Rotate180);

    disp.reset(&mut rst, &mut delay).unwrap();
    disp.init().unwrap();
    disp.flush().unwrap();

    // Loads an 86x64px image encoded in LE (Little Endian) format. This image is a 16BPP image of
    // the Rust mascot, Ferris.
    let im = ImageRawLE::new(include_bytes!("./ferris.raw"), 86, 64);

    Image::new(&im, Point::new((96 - 86) / 2, 0))
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    led_red.set_low().ok();
    led_green.set_high().ok();

    loop {
        continue;
    }
}

#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use litex_hal::prelude::*;
use litex_hal::pac;
use litex_hal::clock::Clocks;
use litex_hal::delay::Delay;
use litex_hal::println;
use litex_hal::serial::Serial;


#[entry]
fn main() -> ! {
    let clocks = Clocks::new();

    let uart = unsafe { pac::uart::UART::conjure() };
    let uart_phy = unsafe { pac::uart_phy::UART_PHY::conjure() };
    let serial = Serial::new(uart, uart_phy, 115200.bps(), &clocks);
    litex_hal::stdout::set_serial(serial);

    println!("Firmware starting...");
    println!("Core frequency: {} MHz", clocks.sysclk().0 / 1_000_000);

    let gpio = unsafe { pac::gpio::GPIO::conjure() };
    let mut pins = gpio.split();

    let mut led_red = pins.pin(24).unwrap().into_push_pull_output();
    let mut led_green = pins.pin(25).unwrap().into_push_pull_output();
    led_red.set_high().ok();

    let timer = pac::timer0::TIMER0::take().unwrap();
    let mut delay = Delay::new(timer, &clocks);

    let mut i = 0u8;
    loop {
        i = i.wrapping_add(1);
        println!("..{}", i);

        led_green.toggle().ok();
        led_red.toggle().ok();

        delay.delay_ms(500);
    }
}

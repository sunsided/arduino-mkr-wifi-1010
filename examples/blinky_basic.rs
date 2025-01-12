// SPDX-License-Identifier: MIT or APACHE-2.0
// Original Author: Osvald Ivarsson (https://github.com/Foosvald)
// Source: https://github.com/atsamd-rs/atsamd/pull/606

#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt_rtt as _;

#[allow(unused_imports)]
use panic_probe as _;

use firmware as bsp;
use bsp::hal;
use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led: bsp::Led = pins.d6.into();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        delay.delay_ms(500u32);
        led.set_high().unwrap();
        defmt::info!("on");

        delay.delay_ms(1000u32);
        led.set_low().unwrap();
        defmt::info!("off");
    }
}

// SPDX-License-Identifier: MIT or APACHE-2.0
// Original Author: Osvald Ivarsson (https://github.com/Foosvald)
// Source: https://github.com/atsamd-rs/atsamd/pull/606

#![no_std]
#![no_main]

use atsamd_hal::nb;
use bsp::hal;
use bsp::pac;
use firmware as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;

use pac::{CorePeripherals, Peripherals};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);
    let mut delay = hal::delay::Delay::new(core.SYST, &mut clocks);

    let mut red_led: bsp::Led = pins.d6.into_push_pull_output();

    // Setup UART peripheral.
    let mut uart = bsp::uart(
        &mut clocks,
        9600.Hz(),
        peripherals.sercom5,
        &mut peripherals.pm,
        pins.rx,
        pins.tx,
    );

    // Write out a message on start up.
    for byte in b"Hello world!\r\n" {
        nb::block!(uart.write(*byte)).unwrap();
    }

    loop {
        match uart.read() {
            Ok(byte) => {
                // Echo all received characters.
                nb::block!(uart.write(byte)).unwrap();

                // Blink the red LED to show that a character has arrived.
                red_led.set_high().unwrap();
                delay.delay_ms(2u16);
                red_led.set_low().unwrap();
            }
            Err(_) => delay.delay_ms(5u16),
        };
    }
}

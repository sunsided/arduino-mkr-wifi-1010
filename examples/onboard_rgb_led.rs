// SPDX-License-Identifier: MIT or APACHE-2.0
// Original Author: Osvald Ivarsson (https://github.com/Foosvald)
// Source: https://github.com/atsamd-rs/atsamd/pull/606
//
// This blinks the onboard RGB led through SPI to the NINA-W102 Wi-Fi module
// where the RGB led sits The example uses the wifi_nina library to control the
// WiFi on the NINA-W102 when it runs the Arduino nina-fw firmware which is installed by default on all Arduino MKR 1010s: https://github.com/arduino/nina-fw

#![no_std]
#![no_main]

use atsamd_hal::clock::GClock;
use bsp::hal;
use core::time::Duration;
use atsamd_hal::nb;
use atsamd_hal::time::Nanoseconds;
use firmware as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::timer::{TimerCounter, TimerCounter5};
use wifi_nina::transport::SpiTransport;

const BOOT_DELAY_MS: u16 = 100;

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
    let mut delay = Delay::new(core.SYST, &mut clocks);

    delay.delay_ms(BOOT_DELAY_MS);

    let spi = bsp::nina_spi_master(
        &mut clocks,
        8.MHz(),
        peripherals.sercom4,
        &mut peripherals.pm,
        pins.nina_sck,
        pins.nina_mosi,
        pins.nina_miso,
    );

    let gclk0 = clocks.gclk0();
    let mut timer = Timer::new(peripherals.tc5, &mut clocks, &gclk0, &mut peripherals.pm);

    let spi_transport = SpiTransport::start(
        spi,
        pins.nina_ack.into_floating_input(),
        pins.nina_resetn.into_push_pull_output(),
        pins.nina_cs.into_push_pull_output(),
        |d: Duration| delay.delay_ms(d.as_millis() as u32),
    )
    .unwrap();
    let mut wifi = wifi_nina::Wifi::new(spi_transport);

    // Full power is too strong to look at :)
    // let rgb = [[255, 0, 0], [0, 255, 0], [0, 0, 255], [255, 255, 255], [0, 0,
    // 0]];
    let rgb = [[64, 0, 0], [0, 64, 0], [0, 0, 64], [64, 64, 64], [0, 0, 0]];
    let mut current = 0;

    let mut led_last_toggled = timer.millis();
    wifi.set_led(rgb[current][0], rgb[current][1], rgb[current][2])
        .unwrap();

    loop {
        timer.tick();
        let now = timer.millis();
        if (now - led_last_toggled) > 1000 {
            wifi.set_led(rgb[current][0], rgb[current][1], rgb[current][2])
                .unwrap();
            current = (current + 1) % rgb.len();
            led_last_toggled = now;
        }
    }
}

pub struct Timer {
    tc: TimerCounter5,
    millis: u64,
}

impl Timer {
    pub fn new(
        tc5: bsp::pac::Tc5,
        clocks: &mut GenericClockController,
        gclk0: &GClock,
        pm: &mut bsp::pac::Pm,
    ) -> Self {
        let timer_clock = clocks.tc4_tc5(gclk0).unwrap();
        let mut timer = TimerCounter::tc5_(&timer_clock, tc5, pm);
        atsamd_hal::prelude::_embedded_hal_timer_CountDown::start(&mut timer, Nanoseconds::kHz(1));
        Timer {
            tc: timer,
            millis: 0,
        }
    }

    pub fn tick(&mut self) {
        nb::block!(atsamd_hal::prelude::_embedded_hal_timer_CountDown::wait(&mut self.tc)).ok();
        self.millis += 1;
    }

    pub fn millis(&self) -> u64 {
        self.millis
    }
}

//! Makes the device appear as a USB serial port loop back device.
//! Repeats back all characters sent to it, but in upper case.

// SPDX-License-Identifier: MIT or APACHE-2.0
// Original Author: Osvald Ivarsson (https://github.com/Foosvald)
// Source: https://github.com/atsamd-rs/atsamd/pull/606

#![no_std]
#![no_main]

use bsp::hal;
use bsp::pac;
use firmware as bsp;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;

use pac::Peripherals;

use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.gclk,
        &mut peripherals.pm,
        &mut peripherals.sysctrl,
        &mut peripherals.nvmctrl,
    );

    let pins = bsp::Pins::new(peripherals.port);

    let usb_bus = bsp::usb_allocator(
        peripherals.usb,
        &mut clocks,
        &mut peripherals.pm,
        pins.usb_dm,
        pins.usb_dp,
    );

    let mut serial = SerialPort::new(&usb_bus);
    let mut led: bsp::Led = pins.d6.into();

    let descriptors = [
        StringDescriptors::new(LangID::EN_US)
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST"),
        StringDescriptors::new(LangID::DE)
            .manufacturer("Falsche Firma")
            .product("Serieller Port")
            .serial_number("TEST")
    ];

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0xFFFF, 0x1337))
        .strings(&descriptors).unwrap()
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led.set_high().unwrap(); // Turn on

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        led.set_low().unwrap(); // Turn off
    }
}

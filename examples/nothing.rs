#![no_std]
#![no_main]

use firmware as bsp;
use bsp::hal;

use panic_halt as _;

use bsp::entry;

#[entry]
fn main() -> ! {
    loop {}
}

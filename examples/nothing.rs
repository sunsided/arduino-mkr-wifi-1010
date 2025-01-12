#![no_std]
#![no_main]

use bsp::hal;
use firmware as bsp;

use panic_halt as _;

use bsp::entry;

#[entry]
fn main() -> ! {
    loop {}
}

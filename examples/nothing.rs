#![no_std]
#![no_main]

use firmware as bsp;
use bsp::entry;

#[entry]
fn main() -> ! {
    loop {}
}

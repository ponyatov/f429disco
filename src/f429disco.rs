#![no_std]
#![no_main]

use panic_semihosting as _;
use cortex_m_rt::entry;
use f429disco::*;

#[entry]
fn main() -> ! {
    hprintln!(hello());
    loop {}
}

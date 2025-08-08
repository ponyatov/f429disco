#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use f429disco::*;
use panic_semihosting as _;

#[entry]
fn main() {
    hprintln!("{} ver:{}", ABOUT, VERSION);
    hprintln!("(c) {} <{}> {} {}", AUTHOR, EMAIL, YEAR, LICENSE,);
    // loop {}
}

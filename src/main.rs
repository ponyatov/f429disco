#![cfg_attr(not(target_os = "linux"), no_std)]
#![cfg_attr(not(target_os = "linux"), no_main)]

#[cfg(target_os = "linux")]
fn main() {
    println!("f429disco running on Linux");
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
use panic_semihosting as _;

#[cfg(all(target_arch = "arm", target_os = "none"))]
use cortex_m_rt::entry;

#[cfg(all(target_arch = "arm", target_os = "none"))]
#[entry]
fn main() -> ! {
    loop {}
}

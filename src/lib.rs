#![cfg_attr(not(target_os = "linux"), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

/// single byte
pub type byte = u8;
/// @ref M address (cutten to MCU memory sizes)
pub type addr = u16;
/// generic integer (not too large for tiny MCUs registers & ops)
pub type cell = i32;

/// bytecode memory area size, bytes
pub const Msz: usize = 0x10000;
/// return stack size, addr esses
pub const Rsz: usize = 0x100;
/// data stack size, cells
pub const Dsz: usize = 0x10;

/// platform-specific hello function
#[cfg(target_os = "linux")]
pub fn hello() {
    println!(
        "f429disco Virtual FORTH Machine Msz:{}K Rsz:{} Dsz:{}",
        Msz / 1024,
        Rsz,
        Dsz
    );
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
pub fn hello() {
    use cortex_m_semihosting::hprintln;
    hprintln!(
        "f429disco Virtual FORTH Machine Msz:{}K Rsz:{} Dsz:{}",
        Msz / 1024,
        Rsz,
        Dsz
    ).unwrap();
}

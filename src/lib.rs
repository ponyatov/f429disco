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

pub const ABOUT: &str = const_format::formatcp!(
    "Virtual FORTH Machine Msz:{}K Rsz:{} Dsz:{}",
    Msz / 1024,
    Rsz,
    Dsz
);

pub const VERSION: &str = const_format::formatcp!(
    "{}.{}.{}",
    env!("CARGO_PKG_VERSION_MAJOR"),
    env!("CARGO_PKG_VERSION_MINOR"),
    env!("CARGO_PKG_VERSION_PATCH")
);

pub const AUTHOR: &str = "Dmitry Ponyatov";
pub const EMAIL: &str = "dponyatov@gmail.com";
pub const YEAR: &str = "2025";
pub const LICENSE: &str = "MIT";

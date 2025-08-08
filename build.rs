// build.rs
use std::fs;
use std::path::Path;

#[cfg(feature = "f429disco")]
const MEMORY_X: &str = "hw/f429disco/f429disco.x";
#[cfg(feature = "l496disco")]
const MEMORY_X: &str = "hw/l496disco/l496disco.x";
#[cfg(feature = "netduinoplus2")]
const MEMORY_X: &str = "hw/netduinoplus2/netduinoplus2.x";
#[cfg(feature = "iskra")]
const MEMORY_X: &str = "hw/iskra/iskra.x";
#[cfg(all(feature = "pc", feature = "i386", feature = "none"))]
const MEMORY_X: &str = "hw/qemu386/qemu386.x";
#[cfg(feature = "linux")]
const MEMORY_X: &str = "os/linux/linux.x";

fn main() {
    if Path::new(MEMORY_X).exists() {
        fs::copy(MEMORY_X, "memory.x").unwrap();
        println!("cargo:rerun-if-changed={}", MEMORY_X);
    } else {
        let _ = fs::remove_file("memory.x");
    }
}

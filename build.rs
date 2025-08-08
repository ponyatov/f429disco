// build.rs
use std::fs;
use std::path::Path;

#[cfg!(feature = "f429disco")]
const memoryx = "hw/f429disco/f429disco.x";

fn main() {
    let (board, memoryx) = //
    if cfg!(feature = "f429disco") {
        ("f429disco", "hw/f429disco/f429disco.x")
    } else if cfg!(feature = "netduinoplus2") {
        ("netduinoplus2", "hw/netduinoplus2/netduinoplus2.x")
    } else if cfg!(feature = "l496disco") {
        ("l496disco", "hw/l496disco/l496disco.x")
    } else if cfg!(feature = "pillf103") {
        ("pillf103", "hw/pillf103/pillf103.x")
    } else {
        ("iskra", "hw/iskra/iskra.x")
    };

    if Path::new(memoryx).exists() {
        fs::copy(memoryx, "memory.x").unwrap();
        println!("cargo:rerun-if-changed={}", memoryx);
    } else {
        panic!(
            "Memory layout file not found for board {}: {}",
            board, memoryx
        );
    }
}

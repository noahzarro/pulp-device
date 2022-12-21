// NOTE: Adapted from cortex-m/build.rs

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Put the linker script somewhere the linker can find it
    fs::write(out_dir.join("int_link.x"), include_bytes!("int_link.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir.display());
    println!("cargo:rerun-if-changed=int_link.x");
}

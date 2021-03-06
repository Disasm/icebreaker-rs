use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::env;
use svd2ral::{generate, AddressSize};

const SVD_FILE: &str = "soc.svd";

fn main() {
    // Put the memory definitions somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());
    fs::copy("memory.x", out_dir.join("memory.x")).unwrap();
    println!("cargo:rerun-if-changed=memory.x");


    let xml = &mut String::new();
    File::open(SVD_FILE).unwrap().read_to_string(xml).unwrap();

    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    generate(&xml, crate_dir.join("src"), AddressSize::U32, &["IDENTIFIER_MEM"]).unwrap();

    println!("cargo:rerun-if-changed={}", SVD_FILE);
    println!("cargo:rerun-if-env-changed=FORCE");
}

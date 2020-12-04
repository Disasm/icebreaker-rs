use litex_pac::metadata::*;

fn main() {
    for name in PERIPHERAL_MODULES {
        println!("cargo:rustc-cfg=with_{}", name);
    }
}

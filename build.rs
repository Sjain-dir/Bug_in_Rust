extern crate cmake;

use cmake::Config;

fn main() {
    let des = Config::new("libinterface").build();
    
    println!("cargo:rustc-link-search=native={}", des.display());
    println!("cargo:rustc-link-lib=static=interface");
}
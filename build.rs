extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let dst = cmake::build("libspine-c");
    println!("cargo:rustc-link-search=native={}", dst.join("dist/lib").display());
    println!("cargo:rustc-link-lib=static=spine-c");

    let bindings = bindgen::Builder::default()
        .header("libspine-c/spine-c/include/spine/spine.h")
        .clang_arg("-I./libspine-c/spine-c/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link shared library.
    println!("cargo:rustc-link-lib=X11");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h");
    
    // Use sysroot for the aarch64 target
    if let Ok(r) = env::var("SYSROOT") {
        builder = builder.clang_arg(format!("--sysroot={}", r))
        .clang_arg("--include-directory=/usr/include/")
        .clang_arg("--include-directory=/usr/include/X11/")
        .clang_arg("--include-directory=/usr/lib/gcc-cross/aarch64-linux-gnu/5/include/")
    };

    let bindings  =  builder
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

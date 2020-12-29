extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link shared library.
    println!("cargo:rustc-link-lib=X11");
    println!("cargo:rustc-link-lib=GLESv2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=gles.h");

    binding("headers/wrapper.h", "bindings.rs").expect("Couldn't write bindings!");
    binding("headers/gles.h", "bindings_gles.rs").expect("Couldn't write bindings!");
}

fn binding(src: &str, dest: &str) -> std::io::Result<()> {
    let mut builder = bindgen::Builder::default().header(src);

    // Use sysroot for the aarch64 target
    if let Ok(r) = env::var("SYSROOT") {
        builder = builder
            .clang_arg(format!("--sysroot={}", r))
            .clang_arg("--include-directory=/usr/include/")
            .clang_arg("--include-directory=/usr/include/X11/")
            .clang_arg("--include-directory=/usr/lib/gcc-cross/aarch64-linux-gnu/5/include/")
    };

    let bindings = builder
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join(dest))
}

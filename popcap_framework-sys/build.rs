use std::path::PathBuf;

fn main() {
    let is_windows = std::env::var_os("CARGO_CFG_WINDOWS").is_some();
    let popcap_dir = std::env::var("POPCAP_FRAMEWORK_DIR").expect("Missing POPCAP_FRAMEWORK_DIR");
    let out_dir = std::env::var("OUT_DIR").expect("Missing OUT_DIR");
    let out_path = PathBuf::from(out_dir);
    let mut build = cc::Build::new();
    build.cpp(true);

    let mut bindings = bindgen::Builder::default().clang_arg("-x").clang_arg("c++");

    // Paklib
    if !is_windows {
        panic!("Cannot build paklib on non-windows targets!");
    }
    let mut paklib_base = PathBuf::from(popcap_dir);
    paklib_base.push("osframework\\source\\PakLib");

    let paklib_h = paklib_base.join("PakInterface.h");
    let paklib_cpp = paklib_base.join("PakInterface.cpp");

    println!("cargo:rerun-if-changed={}", paklib_h.display());
    println!("cargo:rerun-if-changed={}", paklib_cpp.display());
    println!("cargo:rerun-if-changed=CPakInterface.h");
    println!("cargo:rerun-if-changed=CPakInterface.cpp");

    bindings = bindings
        .header("CPakInterface.h")
        .clang_arg(format!("-I/{}", paklib_base.display()))
        .whitelist_type("pak_.*")
        .whitelist_function("pak_.*")
        .whitelist_var("FILE_ATTRIBUTE_DIRECTORY");

    build
        .file("CPakInterface.cpp")
        .include(paklib_base)
        .file(paklib_cpp);
    // End Paklib

    let bindings = bindings
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    build.compile("popcap");
}

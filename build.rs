use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=vendor/miniaudio/miniaudio.h");
    println!("cargo:rerun-if-changed=vendor/miniaudio/miniaudio.c");

    if env::var("LIBCLANG_PATH").is_err() {
        println!("cargo:warning=Forcing LIBCLANG_PATH");
        unsafe {
            env::set_var("LIBCLANG_PATH", r"A:\Scoop\apps\llvm\current\bin");
        }
    }

    cc::Build::new()
        .file("vendor/miniaudio/miniaudio.c")
        .include("vendor/miniaudio")
        .compile("miniaudio");

    let bindings = bindgen::Builder::default()
        .header("vendor/miniaudio/miniaudio.h")
        .clang_arg("-IC:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.40.33807/include")
        .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.26100.0/ucrt")
        .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.26100.0/um")
        .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.26100.0/shared")
        .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.26100.0/winrt")
        .clang_arg("-IC:/Program Files (x86)/Windows Kits/10/Include/10.0.26100.0/cppwinrt")
        .generate()
        .expect("Unable to generate bindings for miniaudio");
    let out_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/miniaudio_rs.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings file");
}

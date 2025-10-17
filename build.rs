use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let lib_dir = Path::new("one_loop");
    if lib_dir.exists() {
        // Compile Fortran source into a shared library
        println!("cargo:rerun-if-changed=one_loop/");
        let status = Command::new("gfortran")
            .args(&["-shared", "-fPIC"])
            .args(lib_dir.join("avh_olo.f90").to_str())
            .arg("-o")
            .arg("libavh_olo.so")
            .status()
            .expect("Failed to compile Fortran library");
        assert!(status.success(), "Fortran compilation failed");

        // Tell cargo where to find it
        println!("cargo:rustc-link-search=native=.");
        println!("cargo:rustc-link-lib=dylib=avh_olo");
    } else {
        println!("cargo:warning=Fortran source directory not found. Make sure libavh_olo.so exists.");
        println!("cargo:rustc-link-search=native=.");
        println!("cargo:rustc-link-lib=dylib=avh_olo");
    }
}

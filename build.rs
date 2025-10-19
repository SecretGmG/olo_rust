use std::path::Path;
use std::process::Command;

fn main() {
    let lib_dir = Path::new("oneloop");
    let lib_file = lib_dir.join("libavh_olo.a");

    if !lib_file.exists() {
        println!("libavh_olo.a not found, running create.py to build it...");

        let status = Command::new("python3")
            .arg("create.py")
            .current_dir(lib_dir) // run inside oneloop/
            .status()
            .expect("Failed to run python3 oneloop/create.py");

        if !status.success() {
            panic!("create.py failed, could not generate libavh_olo.a");
        }

        if !lib_file.exists() {
            panic!("libavh_olo.a was not created after running create.py");
        }
    }

    // Link the static library and Fortran runtime
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=avh_olo");
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=quadmath");
}

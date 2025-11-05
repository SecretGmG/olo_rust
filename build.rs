use std::path::Path;
use std::process::Command;

fn main() {
    // Directory where the OneLOop source lives
    let lib_dir = Path::new("oneloop");
    let lib_file = lib_dir.join("libavh_olo.a");

    // Only build if the library doesn't already exist
    if !lib_file.exists() {
        println!("cargo:warning=libavh_olo.a not found, building via create.py...");

        // Check that python3 is available
        if Command::new("python3").arg("--version").status().is_err() {
            panic!("python3 not found! You need Python 3 to build OneLOop.");
        }
        
        // Check that gfortran is available
        if Command::new("gfortran").arg("--version").status().is_err() {
            panic!("gfortran not found! You need gfortran to build OneLOop.");
        }

        // Run the Python build script
        let status = Command::new("python3")
            .arg("create.py")
            .current_dir(lib_dir)
            .status()
            .expect("Failed to run create.py inside oneloop/");

        if !status.success() {
            panic!("create.py failed, could not generate libavh_olo.a");
        }

        if !lib_file.exists() {
            panic!("libavh_olo.a was not created after running create.py");
        }
    }

    // Tell Cargo where to find the static library
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=avh_olo");

    // Link standard Fortran libraries (gfortran, quadmath)
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=quadmath");

    // Re-run build.rs if the OneLOop source changes
    println!("cargo:rerun-if-changed={}", lib_dir.display());
}

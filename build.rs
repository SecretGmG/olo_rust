use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use std::time::SystemTime;

fn main() {
    let lib_dir = Path::new("oneloop");
    let lib_file = lib_dir.join("libavh_olo.a");

    // Only rebuild if libavh_olo.a doesn't exist or is older than any .f90 file
    let rebuild = if !lib_file.exists() {
        true
    } else {
        let lib_mtime = lib_file.metadata()
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let mut source_newer = false;
        if let Ok(entries) = fs::read_dir(lib_dir) {
            for entry in entries.flatten() {
                if let Some(ext) = entry.path().extension() {
                    if ext == "f90" {
                        if let Ok(meta) = entry.metadata() {
                            if let Ok(modified) = meta.modified() {
                                if modified > lib_mtime {
                                    source_newer = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        source_newer
    };

    if rebuild {
        println!("cargo:rerun-if-changed=oneloop/");

        let status = Command::new("python3")
            .current_dir(lib_dir)
            .arg("./create.py")
            .status()
            .expect("Failed to execute create.py for static library");
        assert!(status.success(), "Fortran static library build failed");

        if !lib_file.exists() {
            panic!("libavh_olo.a was not created by create.py");
        }
    }

    // Link the static library and Fortran runtime
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=avh_olo");
    println!("cargo:rustc-link-lib=gfortran");
    println!("cargo:rustc-link-lib=quadmath");
}

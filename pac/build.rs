use std::path::PathBuf;
use std::{env, fs};

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let (device_file, svd_file) = if env::var_os("CARGO_FEATURE_AT32F413").is_some() {
            ("src/at32f413/device.x", "svds/AT32F413xx_v2.svd")
        } else if env::var_os("CARGO_FEATURE_AT32F415").is_some() {
            ("src/at32f415/device.x", "svds/AT32F415xx_v2.svd")
        } else if env::var_os("CARGO_FEATURE_AT32F421").is_some() {
            ("src/at32f421/device.x", "svds/AT32F421xx_v2.svd")
        } else if env::var_os("CARGO_FEATURE_AT32F435").is_some() {
            ("src/at32f435/device.x", "svds/AT32F435xx_v2.svd")
        } else if env::var_os("CARGO_FEATURE_AT32F437").is_some() {
            ("src/at32f437/device.x", "svds/AT32F437xx_v2.svd")
        } else {
            panic!("No device features selected");
        };

        fs::copy(device_file, out.join("device.x")).unwrap();
        fs::copy(svd_file, out.join("peripheral.svd")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

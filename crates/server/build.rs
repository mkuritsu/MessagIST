use std::{env, fs, path::Path};

const FILES: &[&'static str] = &["Rocket.toml"];

fn main() {
    let target_path = env::var_os("OUT_DIR").unwrap();
    let target_path = Path::new(&target_path)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    for file in FILES {
        println!("cargo::rerun-if-changed={}", file);
        println!(
            "cargo::warning=Copying {} to build path {:?}",
            file, target_path
        );
        let src_path = Path::new(file);
        let dst_path = Path::new(&target_path).join(file);
        fs::copy(src_path, dst_path)
            .expect(&format!("Failed to copy {} to output directory!", file));
    }
}

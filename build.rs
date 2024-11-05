use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rustc-link-search=native=./raylib/lib");
    println!("cargo:rustc-link-lib=static=raylib");

    generate_bindings("raylib/include/raylib.h")?;

    Ok(())
}

fn generate_bindings(header_file: &str) -> Result<(), Box<dyn Error>> {
    let builder = bindgen::Builder::default()
        .header(header_file)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

    let bindings = builder.generate()?;

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}

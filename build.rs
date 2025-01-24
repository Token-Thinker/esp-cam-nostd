use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=components/esp32-camera/driver/esp_camera.c");
    println!("cargo:rerun-if-changed=components/esp32-camera/driver/include/esp_camera.h");
    println!("cargo:rerun-if-changed=components/bindings.h");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_toml_path = PathBuf::from(&manifest_dir).join("Cargo.toml");

    let cargo_toml = fs::read_to_string(&cargo_toml_path)
        .expect("Failed to read Cargo.toml");
    let cargo: toml::Value = toml::from_str(&cargo_toml)
        .expect("Failed to parse Cargo.toml");

    // Navigate to package.metadata.extra_components
    let extra_components = &cargo["package"]["metadata"]["extra_components"];

    // Extract individual fields
    let component_dirs = extra_components["component_dirs"]
        .as_str()
        .expect("Missing `component_dirs` in package.metadata.extra_components");
    let bindings_header = extra_components["bindings_header"]
        .as_str()
        .expect("Missing `bindings_header` in package.metadata.extra_components");
    let bindings_module = extra_components["bindings_module"]
        .as_str()
        .expect("Missing `bindings_module` in package.metadata.extra_components");

    println!("cargo:warning=component_dirs: {}", component_dirs);
    println!("cargo:warning=bindings_header: {}", bindings_header);
    println!("cargo:warning=bindings_module: {}", bindings_module);

    let driver_dir = PathBuf::from(&manifest_dir)
        .join(&component_dirs)
        .join("driver");
    let include_dir = driver_dir.join("include");
    let c_file = driver_dir.join("esp_camera.c");

    // Compile the C library
    cc::Build::new()
        .include(&include_dir)
        .file(c_file)
        .compile(bindings_module);


    // Generate Rust bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header(format!("{}/{}", manifest_dir, bindings_header))
        .clang_arg(format!("-I{}/include", include_dir.display())) // Adjust include path as needed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Link the compiled C library
    println!("cargo:rustc-link-lib=static={}", bindings_module);

    // Specify the search path for the linker
    println!(
        "cargo:rustc-link-search={}",
        driver_dir.display()
    );

}

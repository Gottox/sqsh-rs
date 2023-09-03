use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let sqsh_dir = project_dir.join("libsqsh");
    let out_lib_dir = out_dir.join("build");
    let out_lib_path = out_lib_dir.to_str().unwrap();
    let pkg_config_path = out_lib_path.to_string() + "/meson-private";

    println!("cargo:rustc-link-lib=static=sqsh");
    println!("cargo:rustc-link-search=native={}/lib", out_lib_path);
    meson::build(project_dir.join("libsqsh").to_str().unwrap(), &out_lib_path);

    std::env::set_var("PKG_CONFIG_PATH", pkg_config_path);
    pkg_config::Config::new()
        .statik(true)
        .probe("libsqsh")
        .unwrap();

    let subprojects_dir = sqsh_dir.join("subprojects");
    let cextras_dir = fs::read_dir(subprojects_dir)
        .unwrap()
        .filter(|entry| {
            let entry = entry.as_ref().unwrap();
            entry.file_name().to_str().unwrap().starts_with("cextras-")
        })
        .next()
        .unwrap()
        .unwrap()
        .path();

    let bindings = bindgen::Builder::default()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_arg(format!("-I{}/include", sqsh_dir.to_str().unwrap()))
        .clang_arg(format!("-I{}/include", cextras_dir.to_str().unwrap()))
        .clang_arg("-D__attribute__(x)=")
        .header("wrapper.h")
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

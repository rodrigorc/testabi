use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("test.cpp")
        .generate()
        .expect("bindings gen failed");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut build = cc::Build::new();
    //build.compiler("clang");
    build.file("test.cpp");
    build.compile("test");

    println!("cargo:rerun-if-changed=test.cpp");
}

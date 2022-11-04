extern crate cc;

fn main() {
    if cfg!(target_os = "macos") {
        run_macos();
    }
}

fn run_macos() {
    println!("cargo:rustc-link-lib=framework=Cocoa");

    cc::Build::new()
        .file("src/platforms/macos/macos_native.c")
        .flag("-ObjC")
        .compile("skyhook_macos");

    println!("cargo:rerun-if-changed=src/platforms/macos/macos_native.c");
}

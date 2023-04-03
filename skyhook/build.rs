use swift_rs::SwiftLinker;

extern crate cc;

fn main() {
    if cfg!(target_os = "macos") {
        run_macos();
    }
}

fn run_macos() {
    SwiftLinker::new("10.15")
        .with_package("skyhook_mac", "src/platforms/macos/native")
        .link();
}

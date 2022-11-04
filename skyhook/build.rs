extern crate cc;

fn main() {
    if cfg!(target_os = "macos") {
        run_macos();
    }
}

fn run_macos() {
    // TODO
}

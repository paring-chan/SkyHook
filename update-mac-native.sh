cargo build --target x86_64-apple-darwin --release
cargo build --target aarch64-apple-darwin --release

lipo -create \
    target/x86_64-apple-darwin/release/libskyhook_unity.dylib \
    target/aarch64-apple-darwin/release/libskyhook_unity.dylib \
    -output unity/UnityProject/Packages/moe.paring.skyhook/Plugins/skyhook_unity.bundle

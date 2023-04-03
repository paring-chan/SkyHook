// swift-tools-version: 5.7.1
import PackageDescription

let package = Package(
    name: "skyhook_macos_native",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(name: "skyhook_mac", type: .static, targets: ["lib"])
    ],
    targets: [
        .target(name: "lib", path: "src")
    ]
)
// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.
let sha256 = "677c8ea80e4275ac78293158ecb45b50b5c5ed3993447d599d2a75efa5d352c3";
let url = "https://github.com/alexpezzi/stremio-core-swift/releases/download/1.2.62-macos/StremioCore.xcframework.zip";

import PackageDescription

let package = Package(
    name: "StremioCore",
    platforms: [
        .macOS(.v12),
        .macCatalyst(.v13),
        .iOS(.v12),
        .visionOS(.v1),
        .tvOS(.v12)
    ],
    products: [
        .library(
            name: "StremioCore",
            targets: ["StremioCore", "XCFramework"]),
    ],
    dependencies: [
        .package(url: "https://github.com/apple/swift-protobuf.git", from: "1.0.0"),
    ],
    targets: [
        .target(name: "StremioCore",
                dependencies: ["Wrapper", .product(name: "SwiftProtobuf", package: "swift-protobuf")], plugins: [
                    .plugin(name: "SwiftProtobufPlugin", package: "swift-protobuf")
                ]),
        .target(name: "Wrapper", dependencies: []),
        .binaryTarget(name: "XCFramework", url: url, checksum: sha256)
    ]
)

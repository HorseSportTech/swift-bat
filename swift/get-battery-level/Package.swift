// swift-tools-version: 6.0
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "get-battery-level",
    platforms: [
        .iOS(.v12),
    ],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "get-battery-level",
            type: .static,
            targets: ["get-battery-level"]),
    ],
    dependencies: [
        .package(name: "SwiftRs", path: "../swift-rs/"),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .target(
            name: "get-battery-level",
            dependencies: [
                .product(
                    name: "SwiftRs",
                    package: "SwiftRs"
                ),
            ],
            path: "src"),
    ]
)

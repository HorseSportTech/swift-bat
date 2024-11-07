use swift_rs::SwiftLinker;

fn main() {
    // removes the invironment variable because on tauri, the
    // runner by seems to pass in macOS when developing on mac
    // even during a production build
    let temporary = std::env::var("SDKROOT").unwrap();
    std::env::remove_var("SDKROOT");
    // swift-rs has a minimum of macOS 10.13
    // Ensure the same minimum supported macOS version is specified as in your `Package.swift` file.
    SwiftLinker::new("11_15")
        // Only if you are also targetting iOS
        // Ensure the same minimum supported iOS version is specified as in your `Package.swift` file
        .with_ios("12")
        .with_package("get-battery-level", "./swift/get-battery-level")
        .link();

    // add it back again
    std::env::set_var("SDKROOT", temporary);
}
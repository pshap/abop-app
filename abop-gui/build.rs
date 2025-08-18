fn main() {
    // Build script intentionally minimized. Runtime now uses system fonts via assets.rs.
    // If embedding fonts in the future, prefer a feature-gated include_bytes! approach.
    println!("cargo:rerun-if-changed=build.rs");
}

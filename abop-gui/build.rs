fn main() {
    // Build script intentionally minimized. Runtime now uses system fonts via assets.rs.
    // If embedding fonts in the future, prefer a feature-gated include_bytes! approach.
    println!("cargo:rerun-if-changed=build.rs");
    // Track relevant runtime-registered assets and scripts that may affect font setup docs
    println!("cargo:rerun-if-changed=src/assets.rs");
    println!("cargo:rerun-if-changed=assets/fonts/");
    println!("cargo:rerun-if-changed=scripts/setup_fonts.ps1");
    println!("cargo:rerun-if-changed=scripts/update_material_fonts.ps1");
}

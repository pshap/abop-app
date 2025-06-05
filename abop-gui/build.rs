use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    // Get the output directory for generated code
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| {
        eprintln!("Error: OUT_DIR environment variable not set");
        std::process::exit(1);
    });

    // Create fonts.rs directly in the OUT_DIR to ensure it's accessible during builds
    // This avoids the "../../../assets" path that might be inconsistent

    // Re-run if any of these files change
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/fonts");

    // Create a file with the font data included - directly in OUT_DIR
    let font_file = Path::new(&out_dir).join("fonts.rs");

    // Read embedded font files
    let roboto_regular_path = "assets/fonts/roboto/Roboto-Regular.woff2";
    let roboto_medium_path = "assets/fonts/roboto/Roboto-Medium.woff2";
    let roboto_bold_path = "assets/fonts/roboto/Roboto-Bold.woff2";

    println!("cargo:rerun-if-changed={roboto_regular_path}");
    println!("cargo:rerun-if-changed={roboto_medium_path}");
    println!("cargo:rerun-if-changed={roboto_bold_path}");

    let mut roboto_regular_data = Vec::new();
    let mut roboto_medium_data = Vec::new();
    let mut roboto_bold_data = Vec::new();

    fs::File::open(roboto_regular_path)
        .and_then(|mut file| file.read_to_end(&mut roboto_regular_data))
        .unwrap_or_else(|e| {
            eprintln!("Error reading Roboto Regular font: {e}");
            std::process::exit(1);
        });

    fs::File::open(roboto_medium_path)
        .and_then(|mut file| file.read_to_end(&mut roboto_medium_data))
        .unwrap_or_else(|e| {
            eprintln!("Error reading Roboto Medium font: {e}");
            std::process::exit(1);
        });

    fs::File::open(roboto_bold_path)
        .and_then(|mut file| file.read_to_end(&mut roboto_bold_data))
        .unwrap_or_else(|e| {
            eprintln!("Error reading Roboto Bold font: {e}");
            std::process::exit(1);
        });

    // Create font code with embedded data
    let font_code = String::new()
        + r"
        use iced::Font;

        /// Font Awesome 6 Free Solid
        pub const FONT_AWESOME: Font = Font::DEFAULT;

        /// Default system font
        pub const DEFAULT: Font = Font::DEFAULT;

        /// Material Design Roboto fonts
        pub mod roboto {
            use iced::Font;

            /// Roboto Regular font data
            pub const REGULAR_BYTES: &[u8] = &"
        + &format!("{roboto_regular_data:?}")
        + r";
            
            /// Roboto Medium font data
            pub const MEDIUM_BYTES: &[u8] = &"
        + &format!("{roboto_medium_data:?}")
        + r";
            
            /// Roboto Bold font data
            pub const BOLD_BYTES: &[u8] = &"
        + &format!("{roboto_bold_data:?}")
        + r";
            
            /// Get Roboto Regular font
            pub fn regular() -> Font {
                Font::DEFAULT
            }

            /// Get Roboto Medium font
            pub fn medium() -> Font {
                Font::DEFAULT
            }

            /// Get Roboto Bold font
            pub fn bold() -> Font {
                Font::DEFAULT
            }
        }
    ";

    fs::write(font_file, font_code).unwrap_or_else(|e| {
        eprintln!("Error writing font file: {e}");
        std::process::exit(1);
    });
}

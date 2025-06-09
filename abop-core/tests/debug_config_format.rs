//! Test to understand the correct TOML format

use abop_core::config::Config;

#[test]
fn test_default_config_serialization() {
    let config = Config::default();
    let toml_str = toml::to_string_pretty(&config).expect("Failed to serialize config");
    println!("Default config TOML format:\n{}", toml_str);
    
    // Test deserialization to ensure it works
    let _deserialized: Config = toml::from_str(&toml_str).expect("Failed to deserialize config");
}

use polychat_core::plugin::Plugin;

use std::path::PathBuf;
use std::env;

fn get_dummy_plugin() -> String {
    let crate_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").unwrap()
    ).join("tests")
        .join("dummy_plugin")
        .join("target")
        .join("debug");

    if cfg!(target_os = "linux") {
        return crate_dir.join("libdummy_plugin.so").display().to_string();
    } else if cfg!(target_os = "windows") {
        return crate_dir.join("dummy_plugin.dll").display().to_string();
    } else if cfg!(target_os = "macos") {
        return crate_dir.join("libdummy_plugin.dylib").display().to_string();
    }

    panic!("Unsupported Operating System");
}

#[test]
fn load_garbage_path() {
    let plugin = Plugin::new("panic_path.garbage");
    debug_assert!(plugin.is_err(), "Non-existent plugin loaded successfully..");
}

#[test]
fn load_plugin_path() {
    let plugin = Plugin::new(&get_dummy_plugin());
    debug_assert!(!plugin.is_err(), "Error loading plugin: {}", plugin.unwrap_err().to_string());
}

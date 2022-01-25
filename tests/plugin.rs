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
    let plugin = Plugin::new("panic_plugin", "panic_path.garbage");
    assert_eq!(plugin.is_err(), true);
}

#[test]
fn load_plugin_path() {
    let plugin = Plugin::new("dummy_plugin", &get_dummy_plugin());
    assert_eq!(plugin.is_err(), false);
}

#[test]
fn create_account_returns_object() {
    let res: &mut Plugin  = &mut Plugin::new("dummy_plugin", &get_dummy_plugin()).unwrap();
    let account = res.create_account();
    assert_eq!(account.is_err(), false);
}

#[test]
fn print_fn_does_not_err() {
    let plugin : &mut Plugin = &mut Plugin::new("dummy_plugin", &get_dummy_plugin()).unwrap();
    let account = plugin.create_account().unwrap();
    let print_res = plugin.print(account);

    assert_eq!(print_res.is_err(), false);
}
use polychat_core::plugin_manager::PluginManager;

use std::path::PathBuf;
use std::env;

fn get_dummy_plugin() -> String {
    PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").unwrap()
    ).join("tests")
        .join("dummy_plugin")
        .join("target")
        .join("debug").display().to_string()
}

#[test]
fn load_non_absolute_path() {
    let plugin_manager = PluginManager::from("target");
    debug_assert!(plugin_manager.is_err(), "Plugin manager loaded from a non-absolute directory..");
}

#[test]
fn load() {
    let path = get_dummy_plugin();
    let plugin_manager = PluginManager::from(path.as_str());
    debug_assert!(plugin_manager.is_ok(), "Error loading plugin: {}", plugin_manager.unwrap_err());
}

#[test]
fn create_account_for_non_existent_plugin() {
    let path = get_dummy_plugin();
    let mut plugin_manager = PluginManager::from(path.as_str()).unwrap();

    let account = plugin_manager.create_account("garbage");

    debug_assert!(account.is_err(), "Plugin manager found a garbage plugin... We might want to update this test");
}

#[test]
fn create_account_for_existing_plugin() {
    let path = get_dummy_plugin();
    let mut plugin_manager = PluginManager::from(path.as_str()).unwrap();

    let account = plugin_manager.create_account("dummy");

    debug_assert!(account.is_ok(), "Unable to create account for dummy plugin: {}", account.unwrap_err());
}

#[test]
fn destroy_account_for_non_existent_plugin() {
    let path = get_dummy_plugin();
    let mut plugin_manager = PluginManager::from(path.as_str()).unwrap();

    let account = plugin_manager.create_account("dummy").unwrap();

    let res = plugin_manager.delete_account("garbage", account);

    debug_assert!(res.is_err(), "PluginManager found a garbage plugin AND/OR passed the wrong Account to it to be freed");
}

#[test]
fn double_free_never_happens() {
    let path = get_dummy_plugin();
    let mut plugin_manager = PluginManager::from(path.as_str()).unwrap();
    let account = plugin_manager.create_account("dummy").unwrap();

    plugin_manager.delete_account("dummy", account).unwrap();
    let double_free = plugin_manager.delete_account("dummy", account);

    debug_assert!(double_free.is_err(), "Plugin manager double free'd an account");
}

#[test]
fn get_services_returns_services() {
    let path = get_dummy_plugin();
    let plugin_manager = PluginManager::from(path.as_str()).unwrap();
    let services = plugin_manager.get_plugin_names();

    assert_eq!(services, ["dummy"]);
}
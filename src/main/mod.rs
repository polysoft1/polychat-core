extern crate polychat_plugin;


use super::plugin_manager::PluginManager;
use log::info;

use polychat_plugin::plugin::{CoreInterface, Team};
use polychat_plugin::types::Account;


use std::ptr;

#[derive(Debug)]
pub struct Main<'a> {
    plugin_man: Option<PluginManager<'a>>
}

impl<'a> Main<'_> {
    pub fn new() -> Main<'a> {
        Main {
            plugin_man: None
        }
    }
    pub fn init(&self, path: &str, heap: &'a Box<Main<'a>>) -> Result<(), &str> {
        let plugin_man = PluginManager::from(path, heap)?;

        Ok(())
    }
}

impl CoreInterface for Main<'_> {
    /*fn get_teams(&self, _acc: Account) -> *mut Team {
        println!("get_teams called. Returning null...");
        return ptr::null_mut();
    }*/

    fn test(&self, msg: String) {
        info!("Test func called with: {}", msg);
    }
}
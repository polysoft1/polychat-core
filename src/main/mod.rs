extern crate polychat_plugin;

use log::info;

use crate::plugin::Plugin;

use super::plugin_manager::PluginManager;

use polychat_plugin::plugin::CoreInterface;

pub struct Main {
    plugin_manager: Option<PluginManager>
}

impl Main {
    pub fn new() -> Main {
        Main {
            plugin_manager: None
        }
    }

    pub fn init<'a>(&'a mut self, path: &'a str) -> Result<(), &str>{
        let plugin_man = PluginManager::from(path)?;
        self.plugin_manager = Some(plugin_man);

        Ok(())
    }

    pub fn is_initalized(&self) {
        assert!(self.plugin_manager.is_some(), "Main was not initialized!");
    }

    pub fn get_plugin_names(&self) -> Vec<String> {
        self.is_initalized();
        let plugin_manager = self.plugin_manager.as_ref().expect("PluginManager is not initalized");

        return plugin_manager.get_plugin_names();
    }

    pub fn get_plugin_by_name(&self, name: String) -> Result<&Plugin, &str> {
        self.is_initalized();
        let plugin = self.plugin_manager.as_ref().unwrap().get_plugin_by_name(name);

        match plugin {
            None => Err("Could not find plugin"),
            Some(s) => Ok(s)
        }
    }
}

impl CoreInterface for Main {
    fn test(&self, test_msg: String) {
        self.is_initalized();
        info!("[CoreInterface] test function called with {}", test_msg);
    }
}
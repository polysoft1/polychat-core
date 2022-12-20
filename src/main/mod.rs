extern crate polychat_plugin;

use log::info;

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
}

impl CoreInterface for Main {
    fn test(&self, test_msg: String) {
        self.is_initalized();
        info!("Test function called with {}", test_msg);
    }
}
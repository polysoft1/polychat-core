extern crate libloading;
extern crate polychat_plugin;

use libloading::{Library, Error};

use polychat_plugin::plugin::{InitializedPlugin, PluginInfo, INITIALIZE_FN_NAME};
use polychat_plugin::types::Account;

type InitFn = fn (thing: *mut PluginInfo);

#[derive(Debug)]
pub struct Plugin {
    _lib: Library, //Needed to preserve symbol lifetime in plugin_info
    plugin_info: InitializedPlugin
}

impl Plugin {
    /// Creates an initialized Plugin
    /// 
    /// # Arguments
    /// * path - A string slice for an absolute path to a library file (dll/so/dynlib)
    /// 
    /// # Errors
    /// If a Plugin cannot be initialized, a string is returned 
    /// explaining the root cause in an Err type.
    pub fn new(path: &str) -> Result<Plugin, String> {
        let lib_res: Result<Library, Error>;

        unsafe {
            lib_res = Library::new(path);
        }
        
        match lib_res {
            Err(error) => Err(error.to_string()), // Library Errored out
            Ok(lib) => match unsafe { lib.get::<InitFn>(INITIALIZE_FN_NAME.as_bytes()) } {
                Err(error) => Err(error.to_string()), // Finding initialize symbol errored out
                Ok(func) => {
                    let mut plugin_info = PluginInfo::new();
                    func(&mut plugin_info);

                    match InitializedPlugin::new(&plugin_info) {
                        Err(err) => Err(err), // PluginInfo is missing info :(
                        Ok(plugin) => Ok(Plugin {
                            _lib: lib,
                            plugin_info: plugin
                        })
                    }
                }
            }
        }
    }

    pub fn create_account(&self) -> Account {
        (self.plugin_info.create_account)()
    }

    pub fn delete_account(&self, account: Account) {
        (self.plugin_info.destroy_account)(account);
    }

    pub fn print(&self, account: Account) {
        (self.plugin_info.print)(account);
    }
}
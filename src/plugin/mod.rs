extern crate libloading;
extern crate polychat_plugin;

use libloading::{Library, Error};
use log::{info, warn, debug};

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
        info!("Loading lib {}", path);
        unsafe {
            lib_res = Library::new(path);
        }
        
        match lib_res {
            Err(error) => { // Library Errored out
                warn!("Library ({}) failed to load: {}", path, error.to_string());
                return Err(error.to_string());
            },
            Ok(lib) => {
                info!("Successfully loaded library {}", path);
                info!("Loading \"{}\" symbol for initialization", INITIALIZE_FN_NAME);

                match unsafe { lib.get::<InitFn>(INITIALIZE_FN_NAME.as_bytes()) } {
                    Err(error) => { // Finding initialize symbol errored out
                        warn!("Failed to load \"{}\" symbol in {}: {}", INITIALIZE_FN_NAME, path, error.to_string());
                        return Err(error.to_string());
                    },
                    Ok(func) => {
                        info!("Sucessfully loaded symbol \"{}\"", INITIALIZE_FN_NAME);
                        info!("Calling \"{}\"", INITIALIZE_FN_NAME);
                        let mut plugin_info = PluginInfo::new();
                        func(&mut plugin_info);
                        info!("Initializing plugin");
                        match InitializedPlugin::new(&plugin_info) {
                            Err(err) => { // PluginInfo is missing info :(
                                warn!("Could not initialize plugin: {}", err);
                                return Err(err);
                            },
                            Ok(plugin) => {
                                info!("Successfully initialized plugin");
                                return Ok(Plugin {
                                    _lib: lib,
                                    plugin_info: plugin
                                });
                            }
                        }
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
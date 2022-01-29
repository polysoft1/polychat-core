extern crate libloading;
extern crate polychat_plugin;

use libloading::{Library, Error, Symbol};

use polychat_plugin::plugin::{InitializedPlugin, PluginInfo, INITIALIZE_FN_NAME};
use polychat_plugin::types::Account;

type InitFn = fn (thing: *mut PluginInfo);

#[derive(Debug)]
pub struct Plugin {
    name: String,
    _lib: Library, //Needed to preserve symbol lifetime in vtable
    plugin_info: InitializedPlugin
}

impl Plugin {
    pub fn new(name: &str, path: &str) -> Result<Plugin, String> {
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
                            name: name.to_string(),
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
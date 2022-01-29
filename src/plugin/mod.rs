extern crate libloading;
extern crate polychat_plugin;

use libloading::{Library, Error, Symbol};

use polychat_plugin::initialized_plugin::InitializedPlugin;
use polychat_plugin::plugin_info::{INITIALIZE_FN_NAME, PluginInfo};
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
        let init_res: Result<Symbol<InitFn>, Error>;

        unsafe {
            lib_res = Library::new(path);
        }

        if lib_res.is_err() {
            return Err(lib_res.unwrap_err().to_string());
        }

        let lib = lib_res.unwrap();
        
        unsafe {
            init_res = lib.get(INITIALIZE_FN_NAME.as_bytes());
        }

        if init_res.is_err() {
            return Err(init_res.unwrap_err().to_string());
        }

        let mut plugin_info = PluginInfo::new();
        init_res.unwrap()(&mut plugin_info);

        let init_plugin_res = InitializedPlugin::new(&plugin_info);
        if init_plugin_res.is_err() {
            return Err(init_plugin_res.unwrap_err());
        }

        Ok(Plugin {
            name: name.to_string(),
            _lib: lib,
            plugin_info: init_plugin_res.unwrap()
        })
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
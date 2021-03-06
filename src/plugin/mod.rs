extern crate libloading;
extern crate polychat_plugin;

use std::ffi::CString;
use libloading::{Library, Error};
use log::{info, warn};

use polychat_plugin::plugin::{InitializedPlugin, PluginInfo, INITIALIZE_FN_NAME, Message, SendStatus};
use polychat_plugin::types::Account;

type InitFn = fn (thing: *mut PluginInfo);

#[derive(Debug)]
pub struct Plugin {
    _lib: Library, //Needed to preserve symbol lifetime in plugin_info
    plugin_info: InitializedPlugin
}

impl Plugin {
    /// Creates an initialized Plugin, ready for use
    /// 
    /// # Arguments
    /// * path - A string slice for an absolute path to a library file (dll/so/dynlib)
    /// 
    /// # Errors
    /// If a Plugin cannot be initialized for any reason, a string is returned 
    /// explaining the root cause in an Err type.
    pub fn new(path: &str) -> Result<Plugin, String> {
        let lib_res: Result<Library, Error>;
        info!("[{}] Loading lib", path);
        unsafe {
            lib_res = Library::new(path);
        }
        
        match lib_res {
            Err(error) => { // Library Errored out
                warn!("[{}] Library failed to load: {}", path, error.to_string());
                return Err(error.to_string());
            },
            Ok(lib) => {
                info!("[{}] Successfully loaded library", path);
                return new_from_loaded_lib(path, lib);
            }
        }
    }

    pub fn create_account(&self) -> Account {
        (self.plugin_info.create_account)()
    }

    pub fn delete_account(&self, account: Account) {
        (self.plugin_info.destroy_account)(account);
    }

    pub fn post_message(&self, msg_body: String) -> SendStatus {
        let body_cstr = CString::new(msg_body).unwrap();
        let msg = Message {
            body: body_cstr.as_ptr()
        };
        return (self.plugin_info.post_message)(&msg);
    }

    pub fn print(&self, account: Account) {
        (self.plugin_info.print)(account);
    }

    pub fn get_name(&self) -> &String {
        &self.plugin_info.name
    }
}

fn new_from_loaded_lib(path: &str, lib: Library) -> Result<Plugin, String>{
    info!("Loading \"{}\" symbol for initialization", INITIALIZE_FN_NAME);

    match unsafe { lib.get::<InitFn>(INITIALIZE_FN_NAME.as_bytes()) } {
        Err(error) => { // Finding initialize symbol errored out
            warn!("Failed to load \"{}\" from {} symbol: {}", INITIALIZE_FN_NAME, path, error.to_string());
            return Err(error.to_string());
        },
        Ok(func) => {
            info!("[{}] Sucessfully loaded symbol \"{}\"", path, INITIALIZE_FN_NAME);
            info!("[{}] Calling \"{}\"", path, INITIALIZE_FN_NAME);

            let mut plugin_info = PluginInfo::new();
            func(&mut plugin_info);

            info!("[{}] Initializing plugin", path);
            let init_plugin_res = InitializedPlugin::new(&plugin_info); 
            
            if init_plugin_res.is_err() {
                return Err(init_plugin_res.unwrap_err());
            }

            return Ok(Plugin {
                _lib: lib,
                plugin_info: init_plugin_res.unwrap()
            });
        }
    }
}

extern crate libloading;
extern crate polychat_plugin;

use libloading::{Library, Error};
use polychat_plugin::Account;

type CreateAccountFunc = unsafe fn() -> Account;

pub struct Plugin {
    name: String,
    lib: Library,
    accounts: Vec<Account>
}   

impl Plugin {
    pub fn new(name: &str, path: &str) -> Result<Plugin, Error> {
        let lib: Result<Library, Error>;

        unsafe {
            lib = Library::new(path);
        }

        if lib.is_err() {
            return Err(lib.unwrap_err());
        }

        Ok(Plugin {
            name: name.to_string(),
            lib: lib.unwrap(),
            accounts: Vec::<Account>::new()
        })
    }

    pub fn create_account(&self) -> Result<Account, Error> {
        unsafe {
            let func = self.lib.get(b"create_account");

            if func.is_err() {
                return Err(func.unwrap_err());
            }

            Ok(func.unwrap()());
        }
    }
}

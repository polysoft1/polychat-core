extern crate libloading;
extern crate polychat_plugin;

pub mod constants;
pub mod vtable;

use libloading::{Library, Error};
use polychat_plugin::Account;

use vtable::Vtable;

#[derive(Debug)]
pub struct Plugin {
    name: String,
    _lib: Library, //Needed to preserve symbol lifetime in vtable
    vtable: Vtable,
    accounts: Vec<Account>
}

impl Plugin {
    pub fn new(name: &str, path: &str) -> Result<Plugin, Error> {
        let lib_res: Result<Library, Error>;

        unsafe {
            lib_res = Library::new(path);
        }

        if lib_res.is_err() {
            return Err(lib_res.unwrap_err());
        }

        let vtable: Result<Vtable, Error>;
        let lib = lib_res.unwrap();
        unsafe {
            vtable = Vtable::new(&lib);
        }

        if vtable.is_err() {
            return Err(vtable.unwrap_err());
        }

        Ok(Plugin {
            name: name.to_string(),
            _lib: lib,
            vtable: vtable.unwrap(),
            accounts: Vec::<Account>::new()
        })
    }

    pub fn create_account(&mut self) -> Account {
        let account: Account;
        unsafe {
            account = (self.vtable.create_account)();
        }
        self.accounts.push(account);

        return account;
    }

    pub fn print(&self, account: Account) -> () {
        unsafe {
            (self.vtable.print)(account);
        }
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        for account in &self.accounts {
            unsafe {
                (self.vtable.delete_account)(*account);
            }
        }
    }
}

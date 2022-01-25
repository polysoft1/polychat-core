extern crate libloading;
extern crate polychat_plugin;

mod constants;

use libloading::{Library, Error, Symbol};
use polychat_plugin::Account;

use constants::{CREATE_ACCOUNT_FN_NAME,
    DESTROY_ACCOUNT_FN_NAME,
    PRINT_FN_NAME,
    DeleteAccountFunc,
    CreateAccountFunc,
    PrintFunc
};

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

    pub fn create_account(&mut self) -> Result<Account, Error> {
        unsafe {
            let func: Result<Symbol<CreateAccountFunc>, Error> = 
                self.lib.get(CREATE_ACCOUNT_FN_NAME.as_bytes());

            if func.is_err() {
                return Err(func.unwrap_err());
            }

            let account = func.unwrap()();
            self.accounts.push(account);

            return Ok(account);
        }
    }

    pub fn print(&self, account: Account) -> Result<(), Error> {
        let func: Result<Symbol<PrintFunc>, Error>;

        unsafe {
            func = self.lib.get(PRINT_FN_NAME.as_bytes());
        }

        if func.is_err() {
            return Err(func.unwrap_err());
        }
        
        unsafe {
            func.unwrap()(account);
        }
        return Ok(());
    }
}

impl Drop for Plugin {
    fn drop(&mut self) {
        let func: Symbol<DeleteAccountFunc>;
        unsafe {
            func = self.lib.get(DESTROY_ACCOUNT_FN_NAME.as_bytes()).unwrap();
        }
        for account in &self.accounts {
            unsafe {
                func(*account);
            }
        }
    }
}

extern crate libloading;

use super::constants;

use libloading::{Library, Symbol, Error};
#[cfg(unix)]
use libloading::os::unix::Symbol as RawSymbol;

#[derive(Debug)]
pub struct VTable {
    pub create_account: RawSymbol<constants::CreateAccountFunc>,
    pub print: RawSymbol<constants::PrintFunc>,
    pub delete_account: RawSymbol<constants::DeleteAccountFunc>,
}

impl VTable {
    pub unsafe fn new(lib: &Library) -> Result<VTable, Error> {
        let create_account_res: Result<Symbol<constants::CreateAccountFunc>, Error> = 
            lib.get(constants::CREATE_ACCOUNT_FN_NAME.as_bytes());

        let print_res: Result<Symbol<constants::PrintFunc>, Error> =
            lib.get(constants::PRINT_FN_NAME.as_bytes());
        
        let delete_account_res: Result<Symbol<constants::DeleteAccountFunc>, Error> =
            lib.get(constants::DESTROY_ACCOUNT_FN_NAME.as_bytes());

        if create_account_res.is_err() {
            return Err(create_account_res.unwrap_err());
        } else if print_res.is_err() {
            return Err(print_res.unwrap_err());
        } else if delete_account_res.is_err() {
            return Err(delete_account_res.unwrap_err());
        }

        Ok(VTable {
            create_account: create_account_res.unwrap().into_raw(),
            print: print_res.unwrap().into_raw(),
            delete_account: delete_account_res.unwrap().into_raw()
        })
    }
}
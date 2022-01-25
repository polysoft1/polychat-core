extern crate polychat_plugin;

use polychat_plugin::Account;

pub type CreateAccountFunc = unsafe fn() -> Account;
pub type DeleteAccountFunc = unsafe fn(Account);
pub type PrintFunc = unsafe fn(Account);

pub const CREATE_ACCOUNT_FN_NAME: &str = "create_account"; 
pub const DESTROY_ACCOUNT_FN_NAME: &str = "destroy_account"; 
pub const PRINT_FN_NAME: &str = "print";
extern crate libc;
extern crate polychat_plugin;

use std::boxed::Box;


use polychat_plugin::types::Account;
use polychat_plugin::plugin::PluginInfo;

extern "C" fn create_account() -> Account {
    Box::into_raw(Box::new(5)) as Account
}

extern "C" fn print(acc: Account) {
    let data = acc as *mut u8;
    unsafe {
        println!("Hello {}", *data);
    }
}

extern "C" fn destroy_account(acc: Account) {
    unsafe {
        Box::from_raw(acc);
    }
}

#[no_mangle]
unsafe extern "C" fn initialize(info: *mut PluginInfo) {
    (*info).create_account = Some(create_account);
    (*info).destroy_account = Some(destroy_account);
    (*info).print = Some(print);
}
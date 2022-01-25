extern crate libc;
extern crate polychat_plugin;

use std::boxed::Box;

use polychat_plugin::Account;

#[no_mangle]
pub extern "C" fn create_account() -> Account {
    Box::into_raw(Box::new(5)) as Account
}

#[no_mangle]
pub extern "C" fn print(acc: Account) {
    let data = acc as *mut u8;
    unsafe {
        println!("Hello {}", *data);
    }
}

#[no_mangle]
pub extern "C" fn destroy_account(acc: Account) {
    unsafe {
        Box::from_raw(acc);
    }
}
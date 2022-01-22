extern crate libc;

use std::boxed::Box;

use libc::c_void;

type Account = *mut c_void;

#[no_mangle]
pub extern "C" fn create_acount() -> Account {
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
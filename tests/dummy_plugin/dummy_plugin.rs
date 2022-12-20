extern crate libc;
extern crate polychat_plugin;

use std::boxed::Box;
use std::ffi::CString;
use std::ffi::c_char;

use polychat_plugin::types::Account;
use polychat_plugin::plugin::PluginInfo;
use polychat_plugin::plugin::{Message, Conversation};
use polychat_plugin::plugin::SendStatus;
use polychat_plugin::plugin::{AuthResult, AuthMethod};

extern "C" fn create_account() -> Account {
    Box::into_raw(Box::new(5)) as Account
}

extern "C" fn print(acc: Account) {
    let data = acc as *mut u8;
    unsafe {
        println!("Hello {}", *data);
    }
}

extern "C" fn post_message(acc: Account, msg: * const Message) -> SendStatus {
    unsafe {
        println!("Instructed to post message with body {}", *(*msg).body);
    }
    return SendStatus::Sending;
}

extern "C" fn destroy_account(acc: Account) {
    unsafe {
        Box::from_raw(acc);
    }
}

extern "C" fn login(acc: Account, method: * const AuthMethod, passwd: *const c_char) -> AuthResult {
    return AuthResult::Success;
}

extern "C" fn request_messages(acc: Account, conv: Conversation, ts: u64, limit: u32) {}

#[no_mangle]
unsafe extern "C" fn initialize(info: *mut PluginInfo) {
    (*info).name = CString::new("dummy").expect("Could not create CString").into_raw();
    (*info).protocol_name = CString::new("Pseduo-Protocol").expect("Could not create CString").into_raw();
    (*info).create_account = Some(create_account);
    (*info).destroy_account = Some(destroy_account);
    (*info).post_message = Some(post_message);
    (*info).print = Some(print);
    (*info).request_messages = Some(request_messages);
    (*info).login = Some(login);
}

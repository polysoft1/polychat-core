extern crate polychat_plugin;

use polychat_plugin::plugin::{CoreInterface, Team};
use polychat_plugin::types::Account;


use std::ptr;

pub struct Main {

}

impl CoreInterface for Main {
    fn get_teams(&self, _acc: Account) -> *mut Team {
        println!("get_teams called. Returning null...");
        return ptr::null_mut();
    }

    fn test(&self) {
        println!("Test called.");
    }
}
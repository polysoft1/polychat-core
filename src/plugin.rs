extern crate libloading;

use libloading::{Library, Error};

pub struct Plugin {
    name: String,
    lib: Library
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
            lib: lib.unwrap()
        })
    }
}

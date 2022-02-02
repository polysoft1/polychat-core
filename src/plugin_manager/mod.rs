extern crate polychat_plugin;

use std::{collections::HashMap, fs::ReadDir};
use std::path::Path;

use polychat_plugin::types::Account;
use log::{info, warn, error};

use crate::plugin::Plugin;

pub struct PluginManager {
    plugin_map: HashMap<String, Plugin>,
    account_map: HashMap<String, Account>
}

impl PluginManager {
    pub fn new(dir: &Path) -> Result<PluginManager, &str> {
        let dir_check = check_directory(dir);

        if dir_check.is_err() {
            return Err(dir_check.unwrap_err());
        }

        for entry in dir_check.unwrap() {
            if let Ok(entry) = entry {
                info!("Found {}", entry.path().to_str().expect("Could not decode path"));
            }
        }

        Ok(PluginManager {
            plugin_map: HashMap::<String, Plugin>::new(),
            account_map: HashMap::<String, Account>::new()
        })
    }
    
    pub fn from(path: &str) -> Result<PluginManager, &str> {
        let dir = Path::new(path);
        PluginManager::new(dir)
    }
}

fn check_directory(dir: &Path) -> Result<ReadDir, &str> {
    let str_path = dir.to_str().unwrap_or("Unknown path");
    if !dir.is_absolute() {
        error!("Path {} is not absolute", str_path);
        return Err("Path must be absolute");
    }
    if !dir.exists() {
        error!("Directory {} does not exist", str_path);
        return Err("Directory does not exist");
    }
    if !dir.is_dir() {
        error!("Path {} is not a directory", str_path);
        return Err("Path is not a directory");
    }

    let read_dir_res = dir.read_dir();

    if read_dir_res.is_err() {
        let err = read_dir_res.unwrap_err();
        error!("Could not read directory {}: {}", str_path, err.to_string());
        return Err("Could not read directory");
    }

    Ok(read_dir_res.unwrap())
}
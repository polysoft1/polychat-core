extern crate polychat_plugin;
extern crate walkdir;

use std::ffi::OsStr;
use std::{collections::HashMap, fs::ReadDir};
use std::path::Path;
use walkdir::{WalkDir, DirEntry};

use polychat_plugin::types::Account;
use log::{debug, error};

use crate::plugin::Plugin;

#[cfg(target_os = "linux")]
const DYN_LIB_EXTENSION: &str = "so";
#[cfg(target_os = "macos")]
const DYN_LIB_EXTENSION: &str = "dynlib";
#[cfg(target_os = "windows")]
const DYN_LIB_EXTENSION: &str = "dll";

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

        let iter = WalkDir::new(dir).max_depth(2).min_depth(2).follow_links(false).into_iter();

        for plugin_item in iter.filter_entry(|e| is_expected_file(e)) {
            if let Ok(plugin_item) = plugin_item {
                debug!("Found {}", plugin_item.path().to_str().unwrap_or("Unknown Path"));
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

fn is_expected_file(entry: &DirEntry) -> bool {
    let ext = entry.path().extension().unwrap_or(OsStr::new("Unknown"));

    return entry.path().is_file() && ext == DYN_LIB_EXTENSION;
}

fn check_directory(dir: &Path) -> Result<(), &str> {
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

    Ok(())
}
extern crate polychat_plugin;
extern crate walkdir;

use std::{ 
    ffi::OsStr,
    collections::HashMap,
    path::Path
};
use walkdir::{WalkDir, DirEntry};

use polychat_plugin::types::Account;
use log::{debug, error, warn};

use crate::plugin::Plugin;

#[cfg(target_os = "linux")]
const DYN_LIB_EXTENSION: &str = "so";
#[cfg(target_os = "macos")]
const DYN_LIB_EXTENSION: &str = "dynlib";
#[cfg(target_os = "windows")]
const DYN_LIB_EXTENSION: &str = "dll";

type PluginMap = HashMap<String, Plugin>;
type AccountMap = HashMap<String, Vec<Account>>;

pub struct PluginManager {
    plugin_map: PluginMap,
    account_map: AccountMap,
}

impl PluginManager {
    pub fn new(dir: &Path) -> Result<PluginManager, &str> {
        check_directory(dir)?;
        let mut plugin_map = PluginMap::new();

        let iter = WalkDir::new(dir).max_depth(2).min_depth(2).follow_links(false).into_iter();

        for plugin_item in iter.filter_entry(|e| is_expected_file(e)) {
            if let Ok(plugin_item) = plugin_item {
                let path = plugin_item.path().to_str().unwrap_or("Unknown Path");
                debug!("Found {}", path);
                let plugin_res = Plugin::new(path);
                
                match plugin_res {
                    Ok(plugin) => {
                        let name = plugin.get_name();
                        if plugin_map.contains_key(name) {
                            warn!("Duplicate plugin name {}, using the first one found", name);
                        } else {
                            debug!("Adding {} to the manager", name);
                            plugin_map.insert(name.to_owned(), plugin);
                        }
                    },
                    Err(error) => warn!("[{}] Could not load library: {}", path, error.as_str())
                };
            }
        }

        Ok(PluginManager {
            plugin_map,
            account_map: AccountMap::new()
        })
    }
    
    pub fn from(path: &str) -> Result<PluginManager, &str> {
        let dir = Path::new(path);
        PluginManager::new(dir)
    }

    pub fn create_account(&mut self, service_name: &str) -> Result<Account, &str>  {
        let name = service_name.to_string();
        let plugin = get_plugin(&self.plugin_map, service_name)?;
        
        let account = plugin.create_account();
        
        let vec = self.account_map.entry(name).or_insert(Vec::<Account>::new());
        vec.push(account);
        debug!("Created account {:p} at index {} for {}", account, vec.len() - 1, service_name);

        return Ok(account);
    }

    pub fn delete_account(&mut self, service_name: &str, account: Account) -> Result<(), &str> {
        let name = service_name.to_string();
        let plugin = get_plugin(&self.plugin_map, service_name)?;
        let vector = get_account_vec(&mut self.account_map, service_name)?;
        let account_index = vector.iter().position(|x| *x == account);
        
        match account_index {
            None => {
                warn!("Could not find specified account for {}", service_name);
                return Err("Could not find associated account");
            },
            Some(index) => {
                debug!("Removing account {:p} at index {} for plugin {}", account, index, name);
                vector.remove(index);
                plugin.delete_account(account);
            }
        }
        
        return Ok(());
    }

    pub fn get_services(&self) -> Vec<String> {
        let mut output: Vec<String> = Vec::<String>::new();

        for (key, _) in &self.plugin_map {
            output.push(key.clone());
        }

        return output;
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        for (name, plugin) in &self.plugin_map {
            for accounts in self.account_map.get(name) {
                for account in accounts {
                    // Don't need to update the account vector since
                    // this object is getting dropped
                    plugin.delete_account(*account);
                }
            }
        }
    }
}

fn get_plugin<'map>(plugin_map: &'map PluginMap, service_name: &str) -> Result<&'map Plugin, &'static str> {
    return match plugin_map.get(service_name) {
        None => {
            warn!("Could not find service {}", service_name);
            return Err("No such service");
        }
        Some(plugin) => Ok(plugin)
    };
}

fn get_account_vec<'map>(account_map: &'map mut AccountMap, service_name: &str) -> Result<&'map mut Vec<Account>, &'static str> {
    return match account_map.get_mut(service_name) {
        None => {
            warn!("Could not find associated account map for {}", service_name);
            return Err("No accounts available");
        }
        Some(vector) => Ok(vector),
    };
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
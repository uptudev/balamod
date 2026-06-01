use colour::red_ln;
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

#[inline]
#[cfg(target_os = "windows")]
pub fn get_balatro_paths() -> Vec<PathBuf> {
    let path = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam")
        .map(|k| k.get_value("InstallPath"))
        .flatten()
        .map(|str| Path::new(&str))
        .unwrap_or_else(|_| {
            red_ln!("Could not read steam install path from Registry! Trying standard installation path in `Program Files (x86)`");
            return std::env::var("ProgramFiles(x86)")
                .map(|str| 
                    Path::new(&str)
                        .join("Steam")
                        .as_path())
                .unwrap_or(Path::new(""));
        });
    if path.exists() {
        return get_library_folders(path);
    } else {
        red_ln!("Could not find Steam folder!");
        return Vec::new();
    }
}

#[cfg(target_os = "macos")]
const STEAM_LOCATION: [&'static str; 3] = ["Library", "Application Support", "Steam"];

#[cfg(target_os = "linux")]
const STEAM_LOCATION: [&'static str; 3] = [".local", "share", "Steam"];

#[inline]
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn get_balatro_paths() -> Vec<PathBuf> {
    std::env::var("HOME")
        .map(|str| PathBuf::from(str))
        .map(|mut path| {
            path.extend(STEAM_LOCATION); 
            return get_library_folders(&path);
        })
        .unwrap_or_else(|_| {
            red_ln!("Impossible to get your home dir!");
            return Vec::new();
        })
}

fn get_library_folders(steam_path: &Path) -> Vec<PathBuf> {
    std::fs::read_to_string(steam_path.join("steamapps").join("libraryfolders.vdf"))
        .expect("Failed to read libraryfolders.vdf")
        .lines()
        .filter(|line| line.contains("\t\t\"path\"\t\t"))
        .filter_map(|line| 
            line.split("\"")
                .skip(3)
                .next()
                .map(|path| PathBuf::from(path)))
        .map(|mut path| {
            path.push("steamapps");
            path.push("common");
            path.push("Balatro");
            return path;
        })
        .filter(|path| path.exists())
        .collect()
}

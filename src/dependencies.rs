use crate::Error;

#[cfg(target_os = "macos")]
const TAR_FILE: &'static str = "balamod-macos.tar.gz";
#[cfg(target_os = "windows")]
const TAR_FILE: &'static str = "balamod-windows.tar.gz";
#[cfg(target_os = "linux")]
const TAR_FILE: &'static str = "balamod-linux-native.tar.gz";
#[cfg(target_os = "linux")]
const TAR_FILE_PROTON: &'static str = "balamod-linux-proton.tar.gz";
#[cfg(target_os = "macos")]
const LIB_FILE: &'static str = "balalib.dylib";
#[cfg(target_os = "windows")]
const LIB_FILE: &'static str = "balalib.dll";
#[cfg(target_os = "linux")]
const LIB_FILE: &'static str = "balalib.so";
#[cfg(target_os = "linux")]
const LIB_FILE_PROTON: &'static str = "balalib.dll";
const BALAMOD_LUA_RELEASES: &'static str = "https://github.com/balamod/balamod_lua/releases/download/";
const BALAMOD_LUA_LATEST: &'static str = "https://github.com/balamod/balamod_lua/releases/latest/download/";
const BALALIB_RELEASES: &'static str = "https://github.com/balamod/balalib/releases/download/";
const BALALIB_LATEST: &'static str = "https://github.com/balamod/balalib/releases/latest/download/";
const MAIN_PATCH: &'static str = "https://raw.githubusercontent.com/balamod/balamod_lua/main/main.patch.lua";

#[cfg(target_os = "linux")]
fn get_tar_file_name(linux_native: bool) -> &'static str { if linux_native { TAR_FILE } else { TAR_FILE_PROTON } }
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn get_tar_file_name(linux_native: bool) -> &'static str { TAR_FILE }
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
fn get_tar_file_name(linux_native: bool) -> &'static str { panic!("Unsupported OS") }

fn try_download_to_vec(url: &str) -> Result<Vec<u8>, Error> {
    Ok(reqwest::blocking::get(url)?.bytes()?.to_vec())
}

#[inline]
pub fn download_tar(tag: Option<String>, linux_native: bool) -> Result<Vec<u8>, Error> {
    match tag {
        Some(tag) => try_download_to_vec(&(BALAMOD_LUA_RELEASES.to_string() + &tag + &get_tar_file_name(linux_native))),
        None => try_download_to_vec(&(BALAMOD_LUA_LATEST.to_string() + &get_tar_file_name(linux_native))),
    }
}

pub fn get_balalib_name(linux_native: bool) -> &'static str {
    if !cfg!(any(target_os = "macos", target_os = "windows", target_os = "linux")) { panic!("Unsupported OS") }
    else if cfg!(target_os = "linux") && !linux_native { LIB_FILE_PROTON }
    else { LIB_FILE }
}

#[inline]
pub fn download_balalib(tag: Option<String>, linux_native: bool) -> Result<Vec<u8>, Error> {
    match tag {
        Some(tag) => try_download_to_vec(&(BALALIB_RELEASES.to_string() + &tag + &get_tar_file_name(linux_native))),
        None => try_download_to_vec(&(BALALIB_LATEST.to_string() + &get_tar_file_name(linux_native))),
    }
}

pub fn unpack_tar(dir: &str, tar: Vec<u8>, linux_native: bool) -> Result<(), Error> {
    tar::Archive::new(flate2::read::GzDecoder::new(std::io::Cursor::new(tar)))
        .unpack(dir)?;
    let dir_path = std::path::PathBuf::from(dir);
    std::fs::rename(
        dir_path.join(get_tar_file_name(linux_native).split('.').next().unwrap()), 
        dir_path.join("balamod"))?; // rename dir to balamod
    Ok(())
}

#[inline]
pub fn download_patched_main() -> Result<Vec<u8>, Error> { try_download_to_vec(MAIN_PATCH) }

pub fn balamod_version_exists(ver: &str, linux_native: bool) -> bool {
    reqwest::blocking::get(&(BALAMOD_LUA_RELEASES.to_string() + ver + &get_tar_file_name(linux_native)))
        .unwrap()
        .status()
        .as_u16() != 404
}

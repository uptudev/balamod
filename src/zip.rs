use std::{
    io::{ Read, Write, },
    path::{ Path, PathBuf,},
};

mod paths;
pub use paths::get_balatro_paths;
use crate::Error;

#[inline]
#[cfg(target_os = "macos")]
pub fn get_exe_path(path: &Path) -> PathBuf { 
    let mut path = path.join("Balatro.app");
    path.push("Contents");
    path.push("Resources");
    path.push("Balatro.love");
    return path;
}

#[inline]
#[cfg(any(target_os = "windows", target_os = "linux"))]
pub fn get_exe_path(path: &Path) -> PathBuf { path.join("Balatro.exe") }

pub fn get_file_data(path: &Path, file_name: &str) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::with_capacity(4096);
    zip::ZipArchive::new(
        std::fs::File::open(
            get_exe_path(path))?)?
        .by_name(file_name)?
        .read_to_end(&mut buf)?;
    return Ok(buf);
}

pub fn replace_file(
    exe_path: &Path,
    file_name: &str,
    new_contents: &[u8],
) -> Result<(), Error> {
    let exe_data = std::fs::read(exe_path)?;
    let zip_start = exe_data
        .windows(4)
        .position(|window| window == [0x50, 0x4b, 0x03, 0x04])
        .ok_or("ZIP start not found")?;
    let mut reader = zip::ZipArchive::new(std::io::Cursor::new(&exe_data[zip_start..]))?;
    let mut buffer = Vec::from(&exe_data[0..zip_start]);
    buffer.reserve(4096);
    let mut writer = zip::ZipWriter::new(std::io::Cursor::new(&mut buffer[zip_start..]));
    let file_names: Vec<String> = reader.file_names().map(|str| str.to_string()).collect();

    for name in file_names.into_iter().filter(|name| name != file_name) {
        writer.raw_copy_file(reader.by_name(&name)?)?;
    }
    drop(reader);

    writer.start_file(
        file_name, 
        zip::write::FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored))?;
    writer.write_all(new_contents)?;
    writer.finish()?;
    drop(writer);

    std::fs::write(exe_path, buffer)?;
    Ok(())
}

pub fn get_all_files(path: &Path) -> Result<Vec<String>, Error> {
    Ok(
        zip::ZipArchive::new(std::fs::File::open(get_exe_path(path))?)?
            .file_names()
            .map(|str| str.to_string())
            .collect()
    )
}

pub fn get_version(path: &Path) -> Result<String, Error> {
    Ok(String::from_utf8(
        zip::ZipArchive::new(std::fs::File::open(get_exe_path(path))?)?
            .by_name("version.jkr")?
            .bytes()
            .map(|x| x.unwrap_or(0))
            .collect())?
        .lines()
        .nth(1)
        .unwrap_or_else(|| {
            colour::red_ln!("'version.jkr' not found in the archive.");
            return "0.0.0";
        })
        .to_string())
}

#[inline]
pub fn compress_file(input_path: &str, output_path: &str) -> Result<(), Error> {
    Ok(std::fs::File::create(output_path)?
        .write_all(
            &libflate::deflate::Encoder::new(std::fs::read(input_path)?)
                .finish()
                .into_result()?)?)
}

#[inline]
#[cfg(target_os = "macos")]
pub fn get_save_dir(_linux_native: bool) -> PathBuf { 
    PathBuf::from("Users")
        .join(std::env::var("USER").unwrap())
        .join("Library")
        .join("Application Support")
        .join("Balatro")
}

#[inline]
#[cfg(target_os = "windows")]
pub fn get_save_dir(_linux_native: bool) -> PathBuf { 
    PathBuf::from(std::env::var("APPDATA").unwrap())
        .join("Balatro")
}

#[inline]
#[cfg(target_os = "linux")]
pub fn get_save_dir(linux_native: bool) -> PathBuf { 
    if linux_native {
        PathBuf::from(std::env::var("HOME").unwrap())
            .join(".local") 
            .join("share") 
            .join("love") 
            .join("Balatro")
    } else {
        PathBuf::from(std::env::var("HOME").unwrap())
            .join(".local") 
            .join("share") 
            .join("Steam")
            .join("steamapps")
            .join("compatdata")
            .join("2379780")
            .join("pfx")
            .join("drive_c")
            .join("users")
            .join("steamuser")
            .join("AppData")
            .join("Roaming")
            .join("Balatro")
    }
}

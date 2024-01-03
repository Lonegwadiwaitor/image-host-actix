use core::panic;
use std::{fs::{self}, error::Error, path::Path};

use super::config::Config;

pub enum FileType {
    Image,
    Gif,
    Video
}

#[inline]
pub fn ensure_folders_exist() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("public/images")?;
    fs::create_dir_all("public/gifs")?;
    fs::create_dir_all("public/videos")?;

    fs::create_dir_all("private/images")?;
    fs::create_dir_all("private/gifs")?;
    fs::create_dir_all("private/videos")?;

    Ok(())
}

#[inline]
pub fn get_config<'b>() -> Result<&'b Config, Box<dyn Error>> {
    if let Some(v) = super::config::CFG.get() {
        return Ok(v)
    }

    if !Path::new("config.json").exists() {
        let data = serde_json::to_string(&Config{ bind_ip: String::new(), domain_url: String::new(), token: String::new(), private_timeout: 0 }).unwrap();
        fs::write("config.json", data)?;
    }

    let config: Config = serde_json::from_str(&fs::read_to_string("config.json")?)?;

    _ = super::config::CFG.set(config.clone());

    get_config()
}



#[inline]
pub fn file_type_to_directory<'a>(file_type: FileType) -> &'a str {
    match file_type {
        FileType::Image => "images",
        FileType::Gif => "gifs",
        FileType::Video => "videos",
    }
}

#[inline]
pub fn ext_to_file_type<'a>(ext: &str) -> FileType {
    match ext {
        "png" | "jpg" | "jpeg" => FileType::Image,
        "tif" | "gif" => FileType::Gif,
        "mp4" | "avi" | "wmv" | "flv" | "webm" | "mov" => FileType::Video,

        _ => panic!("")
    }
}

#[inline]
pub fn ext_to_directory<'a>(ext: &str) -> &'a str {
    file_type_to_directory(ext_to_file_type(ext))
}
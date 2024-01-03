use std::{error::Error, fs, path::Path, time::UNIX_EPOCH};

use actix_web::{get, Responder, web};

use crate::{compression::brotli, utils::io::ext_to_directory};

#[inline]
pub fn read_and_decompress(path: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let data = fs::read(path)?;

    let decompressed = brotli::decompress(data)?;

    Ok(decompressed)
}

#[get("/{file}")]
pub async fn public_router(file: web::Path<String>) -> impl Responder {
    get(file.to_string(), false).await
}

#[get("/private/{file}")]
pub async fn private_router(file: web::Path<String>) -> impl Responder {
    get(file.to_string(), true).await
}

#[inline]
pub async fn get(file: String, private: bool) -> impl Responder {
    let path = Path::new(&file);

    if private {
        let config = crate::utils::config::CFG.get().unwrap();

        let modified = fs::metadata(path)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

        if modified > config.private_timeout {
            return vec![]
        }
    }

    if let Some(v) = path.extension() {
        let full_path = format!("{}/{}/{}", if private { "private" } else { "public" }, ext_to_directory(v.to_str().unwrap()), file);

        if let Ok(vec) = read_and_decompress(full_path) {
            return vec
        }
    }

    vec![]
}
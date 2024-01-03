use std::{fs, error::Error, path::Path};

use actix_multipart::Multipart;
use actix_web::{post, web::Bytes, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use rand::{distributions::Alphanumeric, Rng};
use serde_derive::Serialize;

use crate::{utils::{http::RequestInfo, self, io::{FileType, file_type_to_directory}}, compression::brotli};

#[derive(Clone, Serialize)]
struct UploadResult {
    status: String,
    files: Vec<UploadedFile>
}

#[derive(Clone, Serialize)]
struct UploadedFile {
    url: String,
    file_type: String,
    private: bool
}

#[inline]
pub fn save_and_compress(path: String, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let compressed = brotli::compress(data)?;

    fs::write(path, compressed)?;

    Ok(())
}

#[post("/upload")]
pub async fn upload_router(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    let request_info: RequestInfo = RequestInfo::new(&req);

    let mut uploaded_files: Vec<UploadedFile> = vec![];

    if !request_info.is_authorized {
        return Ok(utils::http::not_authorized("invalid token provided"))
    }

    let config = crate::utils::config::CFG.get().unwrap();

    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            let data: Bytes = chunk.unwrap();
            let content_type = field.content_disposition();
            let mut file: String;
            

            if let Some(extension) = Path::new(content_type.get_filename().unwrap()).extension() {
                if request_info.private {
                    file = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(8)
                    .map(char::from)
                    .collect();
                file += extension.to_str().unwrap();
                } else {
                    file = String::from(content_type.get_filename().unwrap());
                }


                let url = match request_info.private {
                    true => format!("https://{}/private/{}", config.domain_url, file),
                    false => format!("https://{}/{}", config.domain_url, file),
                };
                if let Err(res) = match extension.to_str().unwrap() {
                    "png" | "jpg" | "jpeg" => {
                        uploaded_files.push(UploadedFile { url, file_type: String::from("image"), private: request_info.private });

                        save_file(&file, &data, FileType::Image, request_info.private)
                    },
                    "tif" | "gif" => {
                        uploaded_files.push(UploadedFile { url, file_type: String::from("gif"), private: request_info.private });

                        save_file(&file, &data, FileType::Gif, request_info.private)
                    },
                    "mp4" | "avi" | "wmv" | "flv" | "webm" | "mov" => {
                        uploaded_files.push(UploadedFile { url, file_type: String::from("video"), private: request_info.private });

                        save_file(&file, &data, FileType::Video, request_info.private)
                    }
        
                    _ => Err(utils::http::internal_server_error("failed to match extension"))
                } {
                    return Ok(res)
                }
            }
        }
    }

    Ok(HttpResponse::Ok().json(UploadResult{ status: String::from("ok"), files: uploaded_files }))
}

fn save_file(file: &str, data: &Bytes, file_type: FileType, private: bool) -> Result<(), HttpResponse> {
    let file_type_directory = file_type_to_directory(file_type);

    let private_directory = if private { "private" } else { "public" };
    
    let path = format!("{}/{}/{}", private_directory, file_type_directory, file);

    save_and_compress(path, data.to_vec()).unwrap();

    Ok(())
}
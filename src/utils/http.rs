use actix_web::{HttpRequest, HttpResponse};
use serde_json::json;

pub struct RequestInfo {
    pub ip: String,
    pub region_code: String,
    pub cf_ray: String,
    pub content_length: u64,
    pub is_authorized: bool,
    pub private: bool
}

impl RequestInfo {
    pub fn new(req: &HttpRequest) -> Self {
        let mut ip: String = String::from("Unknown");
        let mut region_code: String = String::from("Unknown");
        let mut cf_ray: String = String::from("Unknown");
        let mut content_length: u64 = 0u64;
        let mut is_authorized = false;
        let mut private = false;

        if let Some(v) = req.headers().get("CF-Connecting-IP") {
            ip = String::from(
                v
                    .to_str()
                    .unwrap(),
            );
        }

        if let Some(v) = req.headers().get("​​CF-IPCountry") {
            region_code = String::from(
                v
                    .to_str()
                    .unwrap(),
            );
        }

        if let Some(v) = req.headers().get("CF-ray") {
            cf_ray = String::from(
                v
                    .to_str()
                    .unwrap(),
            );
        }

        if let Some(v) = req.headers().get("Content-Length") {
            content_length = v
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        }

        let config = super::config::CFG.get().unwrap();

        if let Some(v) = req.headers().get("token") {
            is_authorized = v.to_str().unwrap() == config.token
        }

        if let Some(v) = req.headers().get("private") {
            private = v.to_str().unwrap() == "true"
        }

        RequestInfo{ ip, region_code, cf_ray, content_length, is_authorized, private }
    }
}

#[inline]
pub fn not_authorized(content: &str) -> HttpResponse {
    HttpResponse::Forbidden().json(json!({ "status": "not authorized", "content": content }))
}

#[inline]
pub fn internal_server_error(content: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(json!({ "status": "internal server error", "content": content }))
}
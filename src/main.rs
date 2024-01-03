use std::{io::BufReader, fs::File, time::Duration, error::Error};

use actix_web::{HttpServer, App};
use rustls::{ServerConfig, Certificate, PrivateKey};
use rustls_pemfile::{certs, pkcs8_private_keys};

mod compression;
mod endpoints;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = utils::io::get_config().expect("Invalid config");
    utils::io::ensure_folders_exist().unwrap();

    HttpServer::new(|| {
        App::new()
            .service(endpoints::upload::upload_router)
            .service(endpoints::get::public_router)
            .service(endpoints::get::private_router)
    })
    .bind_rustls_021(&config.bind_ip, load_rustls_config().unwrap())?
    .keep_alive(Duration::from_secs(75))
    .run()
    .await
}

// taken from SO
#[inline]
fn load_rustls_config() -> Result<ServerConfig, Box<dyn Error>> {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem")?);
    let key_file = &mut BufReader::new(File::open("key.pem")?);

    // convert files to key/cert objects
    let cert_chain = certs(cert_file)?
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)?
        .into_iter()
        .map(PrivateKey)
        .collect();

    Ok(config.with_single_cert(cert_chain, keys.remove(0))?)
}
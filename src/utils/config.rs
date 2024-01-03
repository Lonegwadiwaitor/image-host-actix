use once_cell::sync::OnceCell;
use serde_derive::{Serialize, Deserialize};

pub static CFG: OnceCell<Config> = OnceCell::new();

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub bind_ip: String, // the IP to bind to
    pub domain_url: String, // for example: lonegladiator.dev
    pub token: String, // token header, used for uploading images.
    pub private_timeout: u64 // time in seconds that a private image will stay accessible for 
}
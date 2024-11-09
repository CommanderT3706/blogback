use std::fs;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DbConfig,
    pub site: SiteConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub server_root: String
}

#[derive(Deserialize)]
pub struct DbConfig {
    pub address: String,
    pub port: i32,
    pub db_name: String,
    pub db_user: String,
    pub db_passwd: String,
}

#[derive(Deserialize)]
pub struct SiteConfig {
    pub homepage: String,
    pub posts_page: String,
    pub static_pages: Vec<String>,
    pub static_lastmod: String,
    pub default_changefreq: String
}

pub fn read_config() -> Config {
    let config_path = "/etc/blogback/config.toml";

    let config_content = fs::read_to_string(config_path).expect("Failed to read config file");

    toml::de::from_str(&config_content).expect("Invalid config file")
}
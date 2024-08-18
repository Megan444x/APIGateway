use serde::{Deserialize};
use config::{Config, File, Environment};
use std::env;

#[derive(Debug, Deserialize)]
struct Settings {
    debug_mode: bool,
    database_url: String,
    server_port: u32,
}

impl Settings {
    fn new() -> Self {
        let mut s = Config::default();
        s.merge(File::with_name("Config").required(false)).unwrap();
        s.merge(Environment::with_prefix("APP")).unwrap();
        s.try_into::<Settings>().expect("Failed to parse configuration")
    }
}

fn main() {
    dotenv::dotenv().ok();
    let database_password = env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    println!("Using database password from .env: {}", database_password);
    let settings = Settings::new();
    println!("Current configuration: {:?}", settings);
}
// src/main.rs
include!(concat!(env!("OUT_DIR"), "/env.rs"));

fn main() {
    println!("Config: {:?}", CONFIG);
    println!("Config.service: {:?}", CONFIG.service);
    println!("Config.logger: {:?}", CONFIG.logger);
}
// src/main.rs
include!(concat!(env!("OUT_DIR"), "/env.rs"));
static CONF: &dyn ConfigHelper = &CONFIG;

trait ConfigHelper: Send + Sync {
    fn get_service(&self) -> &str;
    fn get_logger(&self) -> &str;
}

fn main() {
    println!("Config: {:?}", CONFIG);
    println!("Config.service: {:?}", CONF.get_service());
    println!("Config.logger: {:?}", CONF.get_logger());
}


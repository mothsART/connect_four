extern crate toml;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct TomlConfig {
    http: HttpConf,
    ws:   WSConf
}

#[derive(Deserialize)]
struct HttpConf {
    port: Option<u16>,
}

#[derive(Deserialize)]
struct WSConf {
    port: Option<u16>,
}

pub fn port() -> u16 {
    let mut port = 8000;
    let mut path = "/etc/connectfour/connectfour.toml";
    if cfg!(debug_assertions) {
        path = "connectfour.toml";
    }
    let mut f = File::open(path).expect(&*format!(
        "Couldn't open conf file: {}", path
    ));
    let mut contents = String::new();
    f.read_to_string(&mut contents)
     .expect(&*format!(
        "Something went wrong on reading the conf file : {}",
        path
    ));
    let config: TomlConfig;
    match toml::from_str(&contents) {
        Ok(_config) => config = _config,
        Err(why) => panic!(
            "Something went wrong on deserialize the conf file : {} \n{}",
            path,
            why.description()
        )
    }
    match config.http.port {
        Some(_port) => port = _port,
        None => {}
    }
    port
}

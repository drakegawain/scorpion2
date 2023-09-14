extern crate toml;
extern crate serde;
extern crate serde_derive;

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use dirs;
use serde::Deserialize;
use chrono::{Local, Datelike};

#[derive(Debug, Deserialize)]
struct App {
    default: AppConfig,
}


#[derive(Debug, Deserialize)]
struct AppConfig {
    adress: String,
    port: u32,
    id: String,
}


pub fn save(url: String) -> std::io::Result<()> {
    let mut home = dirs::home_dir().unwrap();
    home.push(".scorpion2");
    home.push("url.txt");
    let mut file = File::create(home)?;
    file.write_all(url.as_bytes())?;
    Ok(())
}

pub fn save_ip(url: String, port: i32) -> std::io::Result<()> {
    let mut home = dirs::home_dir().unwrap();
    home.push(".scorpion2");
    home.push("App.toml");
    let data = format!("[default] \nadress = \"{}\" \nport = {}", url, port);
    let fcheck = OpenOptions::new()
        .write(true)
        .open(home.clone());

    match fcheck {
        Ok(mut f) => {
            f.set_len(0)?;
            f.write_all(data.as_bytes())?;
        },
        Err(_) => {
            let mut f = File::create(home)?;
            f.write_all(data.as_bytes())?;
        },
    };

    Ok(())
}


pub fn save_client(url: String, port: i32, id: String) -> std::io::Result<()> {
    let mut home = dirs::home_dir().unwrap();
    home.push(".scorpion2");
    home.push("App.toml");
    let data = format!("[default] \nadress = \"{}\" \nport = {} \nid = {}", url, port, id);
    let fcheck = OpenOptions::new()
        .write(true)
        .open(home.clone());

    match fcheck {
        Ok(mut f) => {
            f.set_len(0)?;
            f.write_all(data.as_bytes())?;
        },
        Err(_) => {
            let mut f = File::create(home)?;
            f.write_all(data.as_bytes())?;
        },
    };

    Ok(())
}



pub fn load() -> String {
    let mut home = dirs::home_dir().unwrap();
    home.push(".scorpion2");
    home.push("url.txt");
    let mut file = File::open(home).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    return content;
}


pub fn load_route() -> String{
    let mut home = dirs::home_dir().unwrap();
    home.push(".scorpion2");
    home.push("App.toml");

    let mut file = File::open(home).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let app_config: App = toml::from_str(&content).unwrap();
    let route = format!("http://{}:{}", app_config.default.adress, app_config.default.port);
    return route; 
    
}

pub fn get_id() -> String{
    let mut home = dirs::home_dir().unwrap();
    home.push(".scorpion2");
    home.push("App.toml");

    let mut file = File::open(home).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let app_config: App = toml::from_str(&content).unwrap();
    return app_config.default.id;
}

pub fn get_date() -> String{

    let current_date = Local::now();
    let date = format!("{:04}-{:02}-{:02}", current_date.year(), current_date.month(), current_date.day());
    return date;
}

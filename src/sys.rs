use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use chrono::{Local, Datelike};

pub fn save(url: String) -> std::io::Result<()> {
    let mut file = File::create("url.txt")?;
    file.write_all(url.as_bytes())?;
    Ok(())
}

pub fn save_ip(url: String, port: i32) -> std::io::Result<()> {
    let data = format!("[default] \nadress = \"{}\" \nport = {}", url, port);
    let fcheck = OpenOptions::new()
        .write(true)
        .open("App.toml");

    match fcheck {
        Ok(mut f) => {
            f.set_len(0)?;
            f.write_all(data.as_bytes())?;
        },
        Err(_) => {
            let mut f = File::create("App.toml")?;
            f.write_all(data.as_bytes())?;
        },
    };

    Ok(())
}


pub fn save_client(url: String, port: i32, id: String) -> std::io::Result<()> {
    let data = format!("[default] \nadress = \"{}\" \nport = {} \nid = {}", url, port, id);
    let fcheck = OpenOptions::new()
        .write(true)
        .open("App.toml");

    match fcheck {
        Ok(mut f) => {
            f.set_len(0)?;
            f.write_all(data.as_bytes())?;
        },
        Err(_) => {
            let mut f = File::create("App.toml")?;
            f.write_all(data.as_bytes())?;
        },
    };

    Ok(())
}




pub fn load() -> String {
    let mut file = File::open("url.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    return content;
}


pub fn load_route() -> String{

    let mut file = File::open("App.toml").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let vec: Vec<&str> = content.split_inclusive("adress = ").collect(); 
    let vec2: Vec<&str> = vec[1].split_inclusive("port = ").collect(); 
    let vec3: Vec<&str> = vec2[0].split_inclusive("\"").collect(); 
    let ip = vec3[1].replace("\"","");
    let port = vec2[1];
    let route = format!("{}:{}", ip, port);
    return route; 
    
}

pub fn get_id() -> String{

    let mut file = File::open("App.toml").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    
    let vec: Vec<&str> = content.split_inclusive("adress = ").collect(); 
    let vec2: Vec<&str> = vec[1].split_inclusive("id = ").collect(); 
    let id = vec2[1];
    return String::from(id); 
 

}

pub fn get_date() -> String{

    let current_date = Local::today();
    let date = format!("{:04}-{:02}-{:02}", current_date.year(), current_date.month(), current_date.day());
    return date;
}

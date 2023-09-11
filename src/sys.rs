use std::fs::File;
use std::io::prelude::*;

pub fn save(url: String) -> std::io::Result<()> {
    let mut file = File::create("url.txt")?;
    file.write_all(url.as_bytes())?;
    Ok(())
}


pub fn load() -> String {
    let mut file = File::open("url.txt").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    return content;
    
}


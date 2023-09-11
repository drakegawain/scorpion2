use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::db;
use crate::sys;
use crate::cat;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command:Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {

    Connection{
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long)]
        save: bool,
        #[arg(short, long)]
        load: bool,
    },
    Test {
        #[arg(short, long)]
        id: String,
        #[arg(short, long)]
        date: String,
        #[arg(short, long)]
        text: String,
        #[arg(short, long)]
        url: String,
    },
    Cat {},
}

pub fn cli() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref(){
        println!("Value for name : {name}");
    }

    match &cli.command {
        Some(Commands::Connection { url, save, load }) =>{
           if url.is_some(){
                let u = url.clone().unwrap();
                let u2 = url.clone().unwrap();
                db::sql_connect(u); 
                if *save == true {
                    let _ = sys::save(u2); 
                    println!("Saved: {}", save);
                }
           }
           if *load == true{
               let f = sys::load();
               db::sql_connect(f.clone()); 
               println!("Loaded on url {} !", f);
           }
        }
        Some(Commands::Test { url, id, date, text }) =>{
           let id = id.clone();
           let date = date.clone();
           let text = text.clone();
           let i = db::Stretch{
               id,
               date,
               text
           };
           let u = url.clone();
           db::sql_insert(u, i).unwrap(); 
           println!("OK !");
        }
        Some(Commands::Cat{} ) =>{
          cat::cat();  
        }
        None => {}

    }
}






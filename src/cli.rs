use clap::{Parser, Subcommand};

use crate::cat::cat;
use crate::db;
use crate::sys;
use crate::server;
use crate::sys::get_id;
use crate::sys::get_date;
use crate::sys::load_route;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
        id: Option<String>,
        #[arg(short, long)]
        date: Option<String>,
        #[arg(short, long)]
        text: Option<String>,
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long)]
        port: Option<i32>,
        #[arg(short, long)]
        load: bool,
    },
    Client {
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long)]
        port: Option<i32>,
        #[arg(short, long)]
        id: Option<String>,
        #[arg(short, long)]
        save: bool,
        #[arg(short, long)]
        load: bool,
    },
    Server{
        #[arg(short, long)]
        url: Option<String>,
        #[arg(short, long)]
        port: Option<i32>,
        #[arg(short, long)]
        load: bool,
    }
}

pub async fn cli() {
    let cli = Cli::parse();

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
        Some(Commands::Test { url, port, id, date, text, load }) =>{

           if *load == true{
               println!("{}", load_route()); 
           }
        }
        Some(Commands::Client{ url, port, id, save, load } ) =>{
           if url.is_some(){
                let u = url.clone().unwrap();
                let p = port.clone().unwrap();
                let i = id.clone().unwrap();
                if *save == true {
                    let _ = sys::save_client(u, p, i); 
                    cat();
                }
           }
           if *load == true{
               cat(); 
           }
        }
        Some(Commands::Server{ url, port, load } ) =>{
           if url.is_some(){
                if port.is_some(){
                    let p = port.clone().unwrap();
                    let u = url.clone().unwrap();
                    let _ = sys::save_ip(u, p); 
                    let rocket = server::rocket();
                    rocket.launch().await.unwrap();
                }
           }
           if *load == true{
               let rocket = server::rocket();
               rocket.launch().await.unwrap();

           }
        }
        None => {}

    }
}






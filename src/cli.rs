use clap::{Parser, Subcommand};

use crate::cat::cat;
use crate::sys;
use crate::server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command:Option<Commands>,

}

#[derive(Subcommand)]
enum Commands {

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
        database: Option<String>,
        #[arg(short, long)]
        port: Option<i32>,
        #[arg(short, long)]
        load: bool,
    }
}

pub async fn cli() {
    let cli = Cli::parse();

    match &cli.command {
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
        Some(Commands::Server{ url, database, port, load } ) =>{
           if database.is_some(){
               let _ = sys::save(database.clone().unwrap());
           }
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






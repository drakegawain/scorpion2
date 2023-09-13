use cli::cli;


mod cli;
mod db;
mod sys;
mod cat;
mod server;

#[macro_use] extern crate rocket;

#[tokio::main]
async fn main() {

   cli().await; 

    

}

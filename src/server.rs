use figment::{Figment, providers::{Format, Serialized, Toml}};
use crate::server::rocket::Build;
use rocket::{serde::{Serialize, Deserialize}, fairing::AdHoc};
use rocket::http::Status;
use rocket::{self};
use crate::sys; 
use crate::db;
use dirs;


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    app_value: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config{ app_value: 3, }
    }
}


#[post("/?<id>&<date>&<text>")]
fn index(id: String, date: String, text: String) -> Status{
   let url = sys::load(); 
   let insert = db::Stretch{
       id,
       date,
       text,
   };
   db::sql_insert(url, insert).unwrap();
   Status::Accepted
}

#[launch]
pub fn rocket() -> rocket::Rocket<Build> {

    let mut home = dirs::home_dir().unwrap();
    home.push(".scorpion2");
    home.push("App.toml");


    let figment = Figment::from(rocket::Config::default())
        .merge(Serialized::defaults(Config::default()))
        .merge(Toml::file(home).nested());
        

    rocket::custom(figment).mount("/", routes![index])
        .attach(AdHoc::config::<Config>())
}

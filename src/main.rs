#![feature(is_some_and)]

mod db;
mod pages;
mod schema;
mod authentication;
#[cfg(test)]
mod tests;

use db::Dept;
use dotenvy::dotenv;
use rocket::{routes, tokio::sync::RwLock, fs::FileServer, Rocket, launch, Build};
use pages::{api::*, dashboard::*};


pub const WWW: &'static str = "./wwwsrc/";


#[launch]
fn start() -> Rocket<Build> {
    dotenv().ok();
    
    Dept::register().unwrap();

    rocket::build()
        .mount("/", routes![
            submit_ticket,
            login,
            logout,
            create_user,
            assign_ticket,
            owned_tickets,
            my_tickets,
            get_employees,
            dashboard,
            unassigned_tickets,
            get_msg,
            complete_ticket,
            ])
        .mount("/", FileServer::from(format!("{WWW}static")))
        // a hashmap of all logged in users (effectively)
        .manage(RwLock::new(authentication::Keyring::new()))
}
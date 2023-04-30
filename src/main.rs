#![feature(is_some_and)]

mod db;
mod pages;
mod schema;
mod authentication;
#[cfg(test)]
mod tests;

use dotenvy::dotenv;
use rocket::{routes, tokio::sync::RwLock, fs::FileServer};
use pages::{api::*, dashboard::*};


pub const WWW: &'static str = "./wwwsrc/";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    dotenv().ok();

    let _rocket = rocket::build()
        .mount("/", routes![
            submit_ticket,
            login,
            logout,
            create_user,
            assign_ticket,
            owned_tickets,
            my_tickets,
            dashboard, 
            ])
        .mount("/", FileServer::from(format!("{WWW}www")))
        // a hashmap of all logged in users
        .manage(RwLock::new(authentication::Keyring::new()))
        .launch()
        .await?;
    Ok(())
}

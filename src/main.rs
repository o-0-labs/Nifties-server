#[macro_use]
extern crate rocket;

extern crate rbatis;
 
extern crate ring;

mod constant;
mod controller;
mod model;
mod service;
mod utils;

use std::sync::Arc;

use rocket::fairing::AdHoc;
use rbatis::rbatis::Rbatis;
use rbatis::db::DBPoolOptions;

use crate::controller::common_controller::{unvalid_token, general_not_found, CORS};
use crate::controller::test_controller::{insert, query, update, delete};
use crate::controller::login_controller::{login, register};
use crate::constant::MYSQL_URL;

//pub const MYSQL_URL: &'static str = "mysql://nft:nft@101.33.60.164:3306/nft";

#[rocket::main]
async fn main()  {
     
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("linking database...");

    let rb = Rbatis::new();
    let mut opt =DBPoolOptions::new();
    opt.max_connections=10;
    rb.link_opt(MYSQL_URL,opt).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);

    info!("linking database successful!");

  

    if let Err(e) = rocket::build()
                    .register("/",catchers![unvalid_token,general_not_found])
                    .mount("/", routes![query,insert,update,delete,login,register])
                    //.mount("/session", session_controller::routes())
                    .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
                        rocket.manage(rb)
                    }))
                    .attach(CORS)
                    .launch()
                    .await {
                        println!("Whoops! Rocket didn't launch!");
                        // We drop the error to get a Rocket-formatted panic.
                        drop(e);
                    };
}
#[macro_use]
extern crate rocket;

extern crate rbatis;
 
extern crate ring;

use std::sync::Arc;

use rocket::fairing::AdHoc;
use rbatis::rbatis::Rbatis;
use rbatis::db::DBPoolOptions;
use rocket_dyn_templates::Template;

mod constant;
mod controller;
mod model;
mod service;
mod utils;

use crate::controller::common_controller::{unvalid_token, CORS};
use crate::controller::session_controller;
use crate::controller::test_controller::{insert, query, update, delete};


pub const MYSQL_URL: &'static str = "mysql://nft:nft@101.33.60.164:3306/nft";

#[rocket::main]
async fn main()  {
     
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    info!("linking database...");

    let rb = Rbatis::new();
    let mut opt =DBPoolOptions::new();
    opt.max_connections=100;
    rb.link_opt(MYSQL_URL,opt).await.expect("rbatis link database fail");
    let rb = Arc::new(rb);

    info!("linking database successful!");

    if let Err(e) = rocket::build()
                    .register("/",catchers![unvalid_token])
                    .mount("/", routes![query,insert,update,delete])
                    .mount("/session", session_controller::routes())
                    .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
                        rocket.manage(rb)
                    }))
                    .attach(CORS)
                    .attach(Template::fairing())
                    .launch()
                    .await {
                        println!("Whoops! Rocket didn't launch!");
                        // We drop the error to get a Rocket-formatted panic.
                        drop(e);
                    };
}
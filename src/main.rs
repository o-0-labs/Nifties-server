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

use rocket::{fairing::AdHoc, fs::{FileServer, relative}};
use rbatis::rbatis::Rbatis;
use rbatis::db::DBPoolOptions;

use crate::controller::common_controller::{unvalid_token, general_not_found, CORS, static_source,upload};
use crate::controller::test_controller::{insert, query, update, delete};
use crate::controller::login_controller::{login, register};
use crate::constant::MYSQL_URL;
use crate::controller::event_controller::{event_query, event_add, event_view, event_like};
use crate::controller::hackathon_controller::{hackathon_count,hackathon_query,hackathon_join};
use crate::controller::grants_controller::{grants_query,grants_add};


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
                    .mount("/", routes![query,insert,update,delete,static_source,upload,login,register,
                    event_query,event_add,event_view,event_like,
                    hackathon_count,hackathon_query,hackathon_join,
                    grants_query,grants_add
                    ])
                    .mount("/", FileServer::from(relative!("img")))
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
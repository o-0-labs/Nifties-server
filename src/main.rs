#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rbatis;
 
extern crate ring;


use serde_json::{Value, json};
// use solana_program::account_info::Account;
// use serde::de::DeserializeSeed;

use std::sync::Arc;
use std::time::Duration;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use rocket::fairing::AdHoc;
use rocket::{ State, routes, };
use rocket::fairing::{Fairing, Info, Kind};
use rocket::response::status;
use rocket::http::Status;
use rocket::http::Header;
use rocket::{Request, Response};
// use rocket::serde::json::Json;
use rocket::form::validate::Contains;

 
 

use rbatis::executor::Executor;
// use rbatis::PageRequest;
use rbatis::plugin::page::{Page, PageRequest};
use rbatis::crud::{CRUD, Skip};
use rbatis::rbatis::{Rbatis};


 

 

 

 

 
//解决跨域
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/hello")]
async fn hello() -> String {
//     ApiResponse::custom_error(MESSAGE_4000.to_string(),4000)
"hello".to_string()
}


 
#[catch(401)]
fn unvalid_token( ) -> String {
//     ApiResponse::custom_error(MESSAGE_4000.to_string(),4000)
"hello".to_string()
}


#[rocket::main]
async fn main()  {
     
    // let rb = Rbatis::new();
    // rb.link(MYSQL_URL).await.expect("rbatis link database fail");
    // let rb = Arc::new(rb);
    // let manager = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    // let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();


        rocket::build()
        .register("/",catchers![unvalid_token])
        .mount("/", routes![hello])
    
        // .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
        //     rocket.manage(rb)
        // }))
        // .manage(pool)
        .attach(CORS)
        .launch()
        .await;
}
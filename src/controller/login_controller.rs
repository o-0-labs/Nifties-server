
use rbatis::rbatis::Rbatis;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use std::sync::Arc;
use tweetnacl_rs::TweetNacl;
use tweetnacl_rs::{from_hex};
use rocket_json_response::{JSONResponse};


use crate::model::common_model::{Token, UserAuth, User};
use crate::service::common_service;
use crate::utils::util;


#[post("/login", format = "json", data = "<user_auth>")]
pub async fn login(rb: &State<Arc<Rbatis>>, user_auth: Json<UserAuth>) -> JSONResponse<'static, Value> {
    info!("login parameter, {:?}",user_auth);

    let user = user_auth.into_inner();

    let pub_key = from_hex(&user.pub_key);
    if pub_key.verify(user.sign_msg, &from_hex(&user.signature)) {
        match common_service::check_user(rb, user.pub_key).await {
            Ok(u) => match common_service::get_token(u).await {
                Ok(u) => {
                    info!("login return ok, {:?}",u);
                    JSONResponse::ok(json!(u))
                },
                Err(_) => {
                    let msg = "get token Fail!";
                    error!("login return err, {}",msg);
                    JSONResponse::err(3,json!({"msg": format!("{}", msg) }))
                },
            },
            Err(_) => {
                let msg = "check user Fail!";
                error!("login return err, {}",msg);
                JSONResponse::err(2,json!({"msg": format!("{}", msg) }))
            },
        }

    } else {
        let msg = "verify Fail!";
        error!("login return err, {}",msg);
        JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
    }
}


#[post("/register", format = "json", data = "<user>")]
pub async fn register(_auth: Token, user: Json<User>, rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("register parameter, {:?}",user);

    let mut u = user.into_inner();

    let sub = format!("{}&{}", &u.user_id, &u.pub_key);
    
    if !sub.eq(&_auth.sub){
        let msg = "token Error!";
        error!("register return err, {}",msg);
        return JSONResponse::err(9,json!({"msg": format!("{}", msg) }))
    }

    if util::is_empty(&u.user_name){
        let msg = "missing username!";
        error!("register return err, {}",msg);
        return JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
    }

    if util::is_empty(&u.email){
        let msg = "missing email!";
        error!("register return err, {}",msg);
        return JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
    }

    if util::is_empty(&u.profile_photo){
        u.profile_photo = Some("xxx.png".to_string());
    }

    
    match common_service::register(rb,u).await{
        Ok(u) => {
            info!("login return ok, {:?}",u);
            JSONResponse::ok(json!(u))
        },
        Err(_) => {
            let msg = "update Fail!";
            error!("register return err, {}",msg);
            JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
        },
    }

}

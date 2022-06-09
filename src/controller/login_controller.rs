use rbatis::rbatis::Rbatis;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use std::sync::Arc;
use rocket_json_response::JSONResponse;


use crate::model::common_model::{Token, UserAuth, User};
use crate::service::common_service;
use crate::utils::util;


#[post("/login", format = "json", data = "<user_auth>")]
pub async fn login(rb: &State<Arc<Rbatis>>, user_auth: Json<UserAuth>) -> JSONResponse<'static, Value> {
    info!("login parameter, {:?}",user_auth);

    let user = user_auth.into_inner();

    //let pub_key = from_hex(&user.pub_key);
    if common_service::verify(&user) {
        match common_service::check_user(rb, user.call_name).await {
            Ok(u) => match common_service::get_token(u).await {
                Ok(u) => {
                    info!("login return ok, {:?}",u);
                    JSONResponse::ok(json!(u))
                },
                Err(_) => {
                    let msg = "get token Fail!";
                    error!("login return err, {}",msg);
                    JSONResponse::err(3,json!({"msg": msg }))
                },
            },
            Err(_) => {
                let msg = "check user Fail!";
                error!("login return err, {}",msg);
                JSONResponse::err(2,json!({"msg": msg }))
            },
        }

    } else {
        let msg = "verify Fail!";
        error!("login return err, {}",msg);
        JSONResponse::err(1,json!({"msg": msg }))
    }
}


#[post("/register", format = "json", data = "<user>")]
pub async fn register(_auth: Token, user: Json<User>, rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("register parameter, {:?}",user);

    let mut u = user.into_inner();

    let sub = format!("{}&{}", &u.user_id, &u.call_name);
    
    if !sub.eq(&_auth.sub){
        let msg = "token Error!";
        error!("register return err, {}",msg);
        return JSONResponse::err(99,json!({"msg": msg }))
    }

    if util::is_empty(&u.user_name){
        let msg = "missing username!";
        error!("register return err, {}",msg);
        return JSONResponse::err(1,json!({"msg": msg }))
    }

    if util::is_empty(&u.email){
        let msg = "missing email!";
        error!("register return err, {}",msg);
        return JSONResponse::err(1,json!({"msg": msg }))
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
            let msg = "user update error!";
            error!("register return err, {}",msg);
            JSONResponse::err(1,json!({"msg": msg }))
        },
    }

}

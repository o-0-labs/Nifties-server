
use rbatis::rbatis::Rbatis;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use std::sync::Arc;
use tweetnacl_rs::TweetNacl;
use rocket_json_response::JSONResponse;


use crate::model::common_model::{Token, UserAuth};
use crate::service::common_service;


// get token
#[post("/login", format = "json", data = "<user_auth>")]
pub async fn login(rb: &State<Arc<Rbatis>>, user_auth: Json<UserAuth>) -> JSONResponse<'static, Value> {
    let user = user_auth.into_inner();

    if user.pub_key.verify(user.sign_msg, &user.signature) {
        match common_service::check_user(rb, user.pub_key).await {
            Ok(u) => match common_service::get_token(&u).await {
                Ok(t) => JSONResponse::ok(json!({"token": format!("{}", t) })),
                Err(_) => {
                    let msg = "get token Fail!";
                    JSONResponse::err(3,json!({"msg": format!("{}", msg) }))
                },
            },
            Err(_) => {
                let msg = "check user Fail!";
                JSONResponse::err(2,json!({"msg": format!("{}", msg) }))
            },
        }

    } else {
        let msg = "verify Fail!";
        JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
    }
}

// get token test
#[post("/token/test")]
pub async fn get_token_test(_auth: Token) -> Value {
    json!({"status":"Auth Success"})
}

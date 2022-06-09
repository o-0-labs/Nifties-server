use std::sync::Arc;

use rbatis::rbatis::Rbatis;
use rocket::{response::{content, Redirect}, serde::json::Json, State};
use rocket_json_response::JSONResponse;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use crate::{constant::{CONSUMER_KEY, CONSUMER_SECRET, OAUTH_CALLBACK, AUTHORIZE_URL, ACCESS_TOKEN, MAIN_URL}, model::{common_model::Token, twitter_model::UserTwitter}, service::twitter_service, utils::util};

#[derive(FromForm,Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Oauth {
    oauth_token: String,
    oauth_verifier: String,
}


#[get("/gettoken?<oauth_token>&<oauth_verifier>")]
pub async fn twitter_token(oauth_token: &str, oauth_verifier: &str) -> content::RawHtml<&'static str> {
    println!("{},{}",oauth_token,oauth_verifier);
    content::RawHtml(r#"
        <p>Hmm... get token</p>
    "#)
}

#[get("/authorize_url")]
pub async fn get_authorize_url(_auth: Token) -> Redirect{

    let con_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

    match egg_mode::auth::request_token(&con_token, OAUTH_CALLBACK).await{
        Ok(t) =>{
            info!("step 1 request_token, oauth_token: {:?}",t);
            let authorize_url = format!("{}?oauth_token={}",AUTHORIZE_URL,t.key);
            //JSONResponse::ok(json!({"authorize_url": format!("{}", authorize_url)}))
            Redirect::to(authorize_url)
        },
        Err(e) => {
            error!("get_authorize_url error! {}",e);
            //JSONResponse::err(1,json!({"msg": format!("{}", e)}))
            let error_url = format!("{}{}",MAIN_URL,"error");
            Redirect::to(error_url)
        },
    }
}


#[post("/access_token", format = "json", data = "<oauth>")]
pub async fn get_access_token(rb: &State<Arc<Rbatis>>, _auth: Token,oauth: Json<Oauth>) -> JSONResponse<'static, Value>{

    let user_id = &_auth.sub[0..32];

    let oauth = oauth.into_inner();
    let params = [("oauth_token",&oauth.oauth_token),("oauth_verifier",&oauth.oauth_verifier)];

    let client = reqwest::Client::new();

    let res = client.post(ACCESS_TOKEN)
    .form(&params)
    .send()
    .await;

    match res {
        Ok(r) => {
            let text = r.text().await;
            match text {
                Ok(t) => {
                    println!("{:?}",t);

                    let mut access_token: Option<String> = None;
                    let mut access_token_secret: Option<String> = None;
                    let mut twitter_user_id: Option<String> = None;
                    let mut screen_name: Option<String> = None;

                    for elem in t.split('&') {
                        let mut kv = elem.splitn(2, '=');
                        match kv.next() {
                            Some("oauth_token") => access_token = kv.next().map(|s| s.to_string()),
                            Some("oauth_token_secret") => access_token_secret = kv.next().map(|s| s.to_string()),
                            //Some("user_id") => twitter_user_id = kv.next().and_then(|s| s.parse::<u64>().ok()),
                            Some("user_id") => twitter_user_id = kv.next().map(|s| s.to_string()),
                            Some("screen_name") => screen_name = kv.next().map(|s| s.to_string()),
                            Some(_) => (),
                            None => {
                                // return Err(error::Error::InvalidResponse(
                                //     "unexpected end of response in access_token",
                                //     None,
                                // ))
                                let msg = "unexpected end of response in access_token";
                                error!("access_token error,{}",msg);
                                return JSONResponse::err(4,json!({"msg": format!("{}", msg)}))
                            }
                        }
                    }

                    if util::is_empty(&access_token) || util::is_empty(&access_token_secret) || util::is_empty(&twitter_user_id) || util::is_empty(&screen_name){
                        return JSONResponse::err(4,json!({"msg": format!("{}", t)}))
                    }

                    let user_twitter = UserTwitter {
                        user_id:user_id.to_string(),
                        twitter_user_id,
                        screen_name,
                        access_token,
                        access_token_secret,
                    };

                    match twitter_service::add_twitter(rb,user_twitter).await {
                        Ok(ut) => {
                            info!("step 3 access_token success,{:?}",ut.screen_name);
                            match ut.screen_name {
                                Some(s) => JSONResponse::ok(json!({"twitter_name": s})),
                                None => {JSONResponse::err(4,json!({"msg": "twitter data missing"}))},
                            }
                        },
                        Err(e) => {
                            error!("access_token error,{}",e);
                            JSONResponse::err(3,json!({"msg": e}))
                        },
                    }

                    
                },
                Err(e) => {
                    error!("access_token error,{}",e);
                    JSONResponse::err(2,json!({"msg": e.to_string()}))
                },
            }
        },
        Err(e) => {
            error!("access_token error,{}",e);
            JSONResponse::err(1,json!({"msg": e.to_string()}))
        }
    }
}


#[post("/remove_twitter")]
pub async fn remove_twitter(rb: &State<Arc<Rbatis>>, _auth: Token) -> JSONResponse<'static, Value>{

    let user_id = &_auth.sub[0..32];

    match twitter_service::remove_twitter(rb, user_id).await {
        Ok(_) => {
            info!("remove_twitter success!");
            JSONResponse::ok(json!({"msg": "remove_twitter success!"}))
        },
        Err(e) => {
            error!("remove_twitter error,{}",e);
            JSONResponse::err(1,json!({"msg": "remove twitter error!"}))
        },
    }
}

#[post("/check_twitter")]
pub async fn check_twitter(rb: &State<Arc<Rbatis>>, _auth: Token) -> JSONResponse<'static, Value>{

    let user_id = &_auth.sub[0..32];

    match twitter_service::check_twitter(rb, user_id).await {
        Ok(r) => {
            match r {
                Some(ut) => {
                    info!("check_twitter success, true");
                    match ut.screen_name {
                        Some(s) => JSONResponse::ok(json!({"twitter_flag": true, "twitter_name": s })),
                        None => {JSONResponse::err(2,json!({"msg": "twitter data missing"}))},
                    }
                    
                },
                None => {
                    info!("check_twitter success, falsse");
                    JSONResponse::ok(json!({"twitter_flag": false}))
                },
            }
        },
        Err(e) => {
            error!("remove_twitter error,{}",e);
            JSONResponse::err(1,json!({"msg": e}))
        },
    }
}
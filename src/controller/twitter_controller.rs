use std::sync::Arc;

use rbatis::rbatis::Rbatis;
use rocket::{response::content, serde::json::Json, State};
use rocket_json_response::JSONResponse;
use serde_json::{Value, json};

use crate::{constant::{CONSUMER_KEY, CONSUMER_SECRET, OAUTH_CALLBACK, AUTHORIZE_URL}, model::{common_model::Token, twitter_model::{Oauth, UserTwitter, Tweets, TimelineParams}}, service::twitter_service, utils::util};


#[get("/gettoken?<oauth_token>&<oauth_verifier>")]
pub async fn twitter_token(oauth_token: &str, oauth_verifier: &str) -> content::RawHtml<&'static str> {
    println!("{},{}",oauth_token,oauth_verifier);
    content::RawHtml(r#"
        <p>Hmm... get token</p>
    "#)
}

#[post("/authorize_url")]
pub async fn get_authorize_url(_auth: Token) -> JSONResponse<'static, Value>{

    let con_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

    match egg_mode::auth::request_token(&con_token, OAUTH_CALLBACK).await {
        Ok(t) =>{
            info!("step 1 request_token, oauth_token: {:?}",t);
            let authorize_url = format!("{}?oauth_token={}",AUTHORIZE_URL,t.key);
            JSONResponse::ok(json!({"authorize_url":  authorize_url}))
            //Redirect::to(authorize_url)
        },
        Err(e) => {
            error!("get_authorize_url error! {}",e);
            JSONResponse::err(1,json!({"msg": "get authorize url error!"}))
            //let error_url = format!("{}{}",MAIN_URL,"error");
            //Redirect::to(error_url)
        },
    }
}


#[post("/access_token", format = "json", data = "<oauth>")]
pub async fn get_access_token(rb: &State<Arc<Rbatis>>, _auth: Token,oauth: Json<Oauth>) -> JSONResponse<'static, Value>{

    let user_id = &_auth.sub[0..32];

    let oauth = oauth.into_inner();

    let res = twitter_service::access(oauth).await;

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

#[post("/tweets", format = "json", data = "<tweets>")]
pub async fn tweets(rb: &State<Arc<Rbatis>>, _auth: Token, tweets : Json<Tweets>) -> JSONResponse<'static, Value>{

    let user_id = &_auth.sub[0..32];

    let tweets = tweets.into_inner();

    if util::is_empty(&tweets.text) {
        error!("tweets error, missing text");
        return JSONResponse::err(1,json!({"msg": "missing text"}));
    }

    match twitter_service::tweets(rb,tweets,user_id).await {
        Ok(id) => {
            match id {
                Value::Null => JSONResponse::err(-1,json!({"msg": "There is no return tweet id, please confirm whether it is successful."})),
                _ => JSONResponse::ok(json!({"id": id})),
            }
            
        },
        Err(e) => JSONResponse::err(2,json!({"msg": e})),
    }
}

#[post("/timeline", format = "json", data = "<timeline>")]
pub async fn get_timeline(rb: &State<Arc<Rbatis>>, _auth: Token, timeline: Json<TimelineParams>) -> JSONResponse<'static, Value>{

    let user_id = &_auth.sub[0..32];
    let timeline = timeline.into_inner();

    let res = twitter_service::get_timeline(rb, user_id, timeline).await;

    match res {
        Ok(json) => {
            match json {
                Value::Null => JSONResponse::err(-1,json!({"msg": "There is no return data, please confirm whether it is successful."})),
                _ => JSONResponse::ok(json),
            }
        },
        Err(e) => {
            JSONResponse::err(1,json!({"msg": e}))
        },
    }
}
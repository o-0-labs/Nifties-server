use rocket::{response::content, serde::json::Json};
use rocket_json_response::JSONResponse;
use serde::{Serialize, Deserialize};
use serde_json::{Value, json};

use crate::{constant::{CONSUMER_KEY, CONSUMER_SECRET, OAUTH_CALLBACK, AUTHORIZE_URL, ACCESS_TOKEN}, model::common_model::Token};

#[derive(FromForm,Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Oauth {
    oauth_token: String,
    oauth_verifier: String,
}


#[get("/gettoken?<oauth>")]
pub async fn twitter_token(oauth: Oauth) -> content::RawHtml<&'static str> {
    println!("{},{}",oauth.oauth_token,oauth.oauth_verifier);
    content::RawHtml(r#"
        <p>Hmm... get token</p>
    "#)
}

#[get("/authorize_url")]
pub async fn get_authorize_url(_auth: Token) -> JSONResponse<'static, Value>{

    let con_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

    match egg_mode::auth::request_token(&con_token, OAUTH_CALLBACK).await{
        Ok(t) =>{
            info!("step 1 request_token, oauth_token: {:?}",t);
            let authorize_url = format!("{}?oauth_token={}",AUTHORIZE_URL,t.key);
            JSONResponse::ok(json!({"authorize_url": format!("{}", authorize_url)}))
        },
        Err(e) => {
            error!("get_authorize_url error! {}",e);
            JSONResponse::err(1,json!({"msg": format!("{}", e)}))
        },
    }
}


#[post("/access_token", format = "json", data = "<oauth>")]
pub async fn get_access_token(_auth: Token,oauth: Json<Oauth>) -> JSONResponse<'static, Value>{

    let oauth = oauth.into_inner();
    let params = [("oauth_token",&oauth.oauth_token),("oauth_verifier",&oauth.oauth_verifier)];

    let client = reqwest::Client::new();

    let res = client.post(ACCESS_TOKEN)
    .form(&params)
    .send()
    .await;

    match res {
        Ok(r) => {
            println!("{:?}",r);
            JSONResponse::ok(json!({"msg": format!("{:?}", r)}))
        },
        Err(e) => {
            println!("{}",e);
            JSONResponse::err(1,json!({"msg": format!("{}", e)}))
        }
    }

    // let request = RequestBuilder::new(Method::POST, "https://api.twitter.com/oauth/access_token")
    //     .oauth_verifier(oauth.oauth_verifier.into())
    //     .request_token(token)
        

    // let (_headers, urlencoded) = raw_request(request).await?;


    // let con_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

    // let request_token = egg_mode::KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

    // let oauth = oauth.into_inner();

    // match egg_mode::auth::access_token(con_token, &oauth.oauth_token, oauth.oauth_verifier).await {
    //     Ok(re) => todo!(),
    //     Err(_) => todo!(),
    // }


    // match egg_mode::auth::request_token(&con_token, OAUTH_CALLBACK).await{
    //     Ok(t) =>{
    //         info!("step 1 request_token, oauth_token: {:?}",t);
    //         let authorize_url = format!("{}?oauth_token={}",AUTHORIZE_URL,t.key);
    //         JSONResponse::ok(json!({"authorize_url": format!("{}", authorize_url)}))
    //     },
    //     Err(e) => {
    //         error!("get_authorize_url error! {}",e);
    //         JSONResponse::err(1,json!({"msg": format!("{}", e)}))
    //     },
    // }
}
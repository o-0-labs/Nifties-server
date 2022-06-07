
use rocket::response::{content, Redirect};
use twapi_reqwest::oauth;

use crate::constant::{CONSUMER_KEY, CONSUMER_SECRET, OAUTH_CALLBACK};


#[get("/gettoken")]
pub async fn twitter_token() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
        <p>Hmm... get token</p>
    "#)
}
//_auth: Token
// #[get("/authorize_url")]
// pub async fn get_authorize_url() -> Result<Redirect,String>{

//     let consumer = oauth_client::Token::new(CONSUMER_KEY,CONSUMER_SECRET);
//     let mut param = HashMap::new();
//     param.insert("oauth_callback".into(), OAUTH_CALLBACK.into());

//     let bytes = oauth_client::get(REQUEST_TOKEN, &consumer, None, Some(&param));

//     if let Ok(bytes) = bytes {
//         let resp = std::str::from_utf8(&bytes);
//         println!("1111111111");
//         if let Ok(resp) = resp{
//             println!("3333333");
//             let param = split_query(&resp);
//             println!("555555555");
//             let token = oauth_client::Token::new(
//                 param.get("oauth_token").unwrap().to_string(),
//                 param.get("oauth_token_secret").unwrap().to_string(),
//             );

//             //let re = twitter_api::get_request_token(&consumer);

//             let url = twitter_api::get_authorize_url(&token);
//             Ok(Redirect::to(url))
            
//         }else{
//             println!("44444444");
//             Err("get resp error".to_string())
//         }

//     }else{
//         println!("2222222222");
//         Err("get token error".to_string())
//     }

// }

// fn split_query(query: &str) -> HashMap<Cow<'_, str>, Cow<'_, str>> {
//     let mut param = HashMap::new();
//     for q in query.split('&') {
//         let mut s = q.splitn(2, '=');
//         let k = s.next().unwrap();
//         let v = s.next().unwrap();
//         let _ = param.insert(k.into(), v.into());
//     }
//     param
// }


#[get("/authorize_url")]
pub async fn get_authorize_url() -> Result<Redirect,String>{
    let x_auth_access_type = "write";

    let res = oauth::request_token(CONSUMER_KEY, CONSUMER_SECRET, OAUTH_CALLBACK, Some(x_auth_access_type)).await;

    match res {
        Ok(map) => {
            println!("{:?}",map);
            Ok(Redirect::to("https://api.twitter.com/oauth/authorize"))
        },
        Err(e) => {
            println!("{}",e);
            Err("111".to_string())
        },
    }

}
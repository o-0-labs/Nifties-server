use rbatis::CRUDTable;
use serde::{Serialize, Deserialize};


#[derive(FromForm,Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Oauth {
    pub oauth_token: String,
    pub oauth_verifier: String,
}

#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserTwitter {
    pub user_id: String,
    pub twitter_user_id: Option<String>,
    pub screen_name:Option<String>,
    pub access_token:Option<String>,
    pub access_token_secret: Option<String>,
}

#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct AccessToken {
    pub twitter_user_id:String,
    pub screen_name:String,
    pub access_token:String,
    pub access_token_secret:String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tweets{
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimelineParams{
    pub max_results:u64,
    pub start_time:Option<String>,
    pub end_time:Option<String>,
    pub since_id:Option<String>,
    pub until_id:Option<String>,
    pub pagination_token:Option<String>,
    pub expansions:Option<String>,
    pub tweet_fields:Option<String>,
    pub user_fields:Option<String>,
    pub media_fields:Option<String>,
    pub place_fields:Option<String>,
    pub poll_fields:Option<String>,
}
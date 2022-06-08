use rbatis::CRUDTable;
use serde::{Serialize, Deserialize};




#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserTwitter {
    pub user_id: String,
    pub twitter_user_id: Option<u64>,
    pub screen_name:Option<String>,
    pub access_token:Option<String>,
    pub access_token_secret: Option<String>,
}
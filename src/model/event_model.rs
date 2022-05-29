use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Event{
    pub event_id: Option<String>,
    pub tag: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
    pub event_address: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub page_view: Option<u64>,
    pub like: Option<u64>,
    pub status: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
}

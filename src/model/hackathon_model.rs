use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct HackathonCount {
    pub happening: u64,
    pub upcoming: u64,
    pub completed: u64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Hackathon {
    pub hackathon_id:Option<String>,
    pub title:Option<String>,
    pub date:Option<String>,
    pub description:Option<String>,
    pub sponsored:Option<String>,
    pub status:Option<String>,
    pub image:Option<String>,
    pub discord_url:Option<String>,
    pub content:Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserHackathon {
    pub user_id: Option<String>,
    pub hackathon_id:Option<String>,
    pub discord:Option<String>,
    pub sharing_email:Option<String>,
    pub agree:Option<String>,
    pub join_time:Option<rbatis::DateTimeNative>,
}
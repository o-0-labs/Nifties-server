use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Grants {
    pub grants_id:Option<String>,
    pub title:Option<String>,
    pub user_id:Option<String>,
    pub user_name:Option<String>,
    pub description:Option<String>,
    //pub total_raised:Option<String>,
    pub logo:Option<String>,
    pub contract_address:Option<String>,
    pub website:Option<String>,
    pub twitter:Option<String>,
    pub bringing:Option<String>,
    pub external_funding:Option<String>,
    pub based:Option<String>,
    pub status:Option<String>,
    pub create_time:Option<rbatis::DateTimeNative>,

}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct GrantsAddress{
    pub contract_address: String,
    pub grants_id: Option<String>,
}
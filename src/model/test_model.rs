extern crate rbatis;


use rbatis::CRUDTable;
use rocket::serde::{Deserialize, Serialize};


#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Article {
    pub id: Option<u64>,
    pub title: String,
    pub author: String,
    pub content: String,
    pub create_time: Option<rbatis::DateTimeNative>,
}


#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ArticleQueryParams {
    pub id: Option<u64>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub page_no: u64,
    pub page_size:u64,
}


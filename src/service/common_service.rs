use std::sync::Arc;

use jsonwebtoken::{encode, EncodingKey, Header};
use rbatis::{push_index, py_sql, rb_py, rbatis::Rbatis, Error};
use rocket::State;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::constant::KEY;
use crate::model::common_model::{User, Claims, UserAuth};
use crate::utils::sha256::Sha256;
use base64;

pub async fn check_user(rb: &State<Arc<Rbatis>>, call_name: String) -> Result<User, Error> {
    
    let re = query_user(rb, &call_name).await;

    match re {
        Err(e) => return Err(e),
        Ok(o) => match o {
            Some(user) => Ok(user),
            None => {
                let uuid = Uuid::new_v4().to_string().replace("-", "");
                let user = User {
                    user_id: uuid,
                    call_name,
                    user_name: None,
                    email: None,
                    profile_photo: None,
                    token: None,
                    create_time: Some(rbatis::DateTimeNative::now()),
                };
                match add_user(rb, &user).await {
                    Ok(_) => return Ok(user),
                    Err(e) => return Err(e),
                }
            }
        },
    }
}

#[py_sql("select * from user where delete_flag = '0' and call_name= #{call_name} ")]
async fn query_user(rb: &State<Arc<Rbatis>>, call_name: &str) -> Option<User> {
    todo!()
}

#[py_sql("insert into user (user_id,call_name,create_time) values (#{user.user_id},#{user.call_name},#{user.create_time}) ")]
async fn add_user(rb: &State<Arc<Rbatis>>, user: &User) -> Result<(), Error> {
    todo!()
}

pub async fn get_token(user: User) -> Result<User, ()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    match encode(
        &Header::default(),
        &Claims {
            sub: format!("{}&{}", user.user_id, user.call_name),
            exp: timestamp + 1800,
        },
        &EncodingKey::from_secret(KEY),
    ) {
        Ok(t) => Ok(User {
            token: Some(t),
            ..user
        }),
        Err(_) => Err(()),
    }
    
}

pub async fn register(rb: &State<Arc<Rbatis>>, user: User) -> Result<User, Error>{
    
    match update_user(rb,&user).await{
        Ok(_) => Ok(user),
        Err(e) => Err(e),
    }
}

#[py_sql("update user set user_name = #{user.user_name}, email = #{user.email}, profile_photo = #{user.profile_photo} where user_id = #{user.user_id} ")]
async fn update_user(rb: &State<Arc<Rbatis>>,user: &User) -> Result<(),Error>{ }


pub fn verify(user: &UserAuth) -> bool{
    let key = "zerolabs-niftiesqweaszQWEASZ@#$@";
    let msg = format!("{}_{}_{}",user.call_name,user.timestamp,key);
    let mut sha256 = Sha256::default();
    sha256.update(msg.as_bytes());
    let res = sha256.finish();
    let result = base64::encode(res);
    //return result;        
    if user.signature.eq(&result) {
        true
    }else{
        false
    }
}
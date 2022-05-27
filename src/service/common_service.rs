use std::sync::Arc;

use jsonwebtoken::{encode, EncodingKey, Header};
use rbatis::{push_index, py_sql, rb_py, rbatis::Rbatis, Error};
use rocket::State;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::constant::KEY;
use crate::model::common_model::{User, Claims};

pub async fn check_user(rb: &State<Arc<Rbatis>>, pub_key: [u8; 32]) -> Result<User, Error> {
    let key_string = format!("{:?}", pub_key);
    let re = query_user(rb, &key_string).await;

    match re {
        Err(e) => return Err(e),
        Ok(o) => match o {
            Some(user) => Ok(user),
            None => {
                let uuid = Uuid::new_v4().to_string().replace("-", "");
                let user = User {
                    user_id: uuid,
                    pub_key: key_string,
                };
                match add_user(rb, &user).await {
                    Ok(_) => return Ok(user),
                    Err(e) => return Err(e),
                }
            }
        },
    }
}

#[py_sql("select * from user where delete_flag = '0' and pub_key= #{pub_key} ")]
pub async fn query_user(rb: &State<Arc<Rbatis>>, pub_key: &str) -> Result<Option<User>, Error> {
    todo!()
}

#[py_sql("insert into user (user_id,pub_key) values (#{user.user_id},#{user.pub_key}) ")]
pub async fn add_user(rb: &State<Arc<Rbatis>>, user: &User) -> Result<(), Error> {
    todo!()
}

pub async fn get_token(user: &User) -> Result<String, ()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let token = match encode(
        &Header::default(),
        &Claims {
            sub: format!("{}&{}", user.user_id, user.pub_key),
            exp: timestamp + 1800,
        },
        &EncodingKey::from_secret(KEY),
    ) {
        Ok(t) => Ok(t),
        Err(_) => Err(()),
    };
    token
}

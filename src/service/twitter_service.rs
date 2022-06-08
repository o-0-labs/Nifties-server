use std::sync::Arc;

use rbatis::{push_index, py_sql, rb_py, rbatis::Rbatis, Error};
use rocket::State;

use crate::model::twitter_model::UserTwitter;




pub async fn add_twitter(rb: &State<Arc<Rbatis>>, user_twitter: UserTwitter) -> Result<UserTwitter, Error>{

    match query_user_twitter(rb, &user_twitter.user_id).await {
        Ok(r) => {
            match r {
                Some(_) => {
                    match update_user_twitter(rb, &user_twitter).await {
                        Ok(_) => Ok(user_twitter),
                        Err(e) => Err(e),
                    }
                },
                None => {
                    match add_user_twitter(rb, &user_twitter).await {
                        Ok(_) => Ok(user_twitter),
                        Err(e) => Err(e),
                    }
                },
            }
        },
        Err(e) => Err(e),
    }
    
}

pub async fn check_twitter(rb: &State<Arc<Rbatis>>, user_id: &str) -> Result<Option<UserTwitter>, Error> {
    query_user_twitter(rb, user_id).await 
}

pub async fn remove_twitter(rb: &State<Arc<Rbatis>>, user_id: &str) -> Result<(),Error>{
    delete_user_twitter(rb,user_id).await
}



#[py_sql("select * from user_twitter where user_id= #{user_id} ")]
pub async fn query_user_twitter(rb: &State<Arc<Rbatis>>, user_id: &str) -> Option<UserTwitter> {
    todo!()
}

#[py_sql("insert into user_twitter (user_id,twitter_user_id,screen_name,access_token,access_token_secret) values (#{user_twitter.user_id},#{user_twitter.twitter_user_id},#{user_twitter.screen_name},#{user_twitter.access_token},#{user_twitter.access_token_secret}) ")]
pub async fn add_user_twitter(rb: &State<Arc<Rbatis>>, user_twitter: &UserTwitter) -> Result<(), Error> {
    todo!()
}

#[py_sql("update user_twitter set twitter_user_id = #{user_twitter.twitter_user_id}, screen_name = #{user_twitter.screen_name}, access_token = #{user_twitter.access_token}, access_token_secret = #{user_twitter.access_token_secret} where user_id = #{user_twitter.user_id} ")]
pub async fn update_user_twitter(rb: &State<Arc<Rbatis>>,user_twitter: &UserTwitter) -> Result<(),Error>{ }


#[py_sql("delete from user_twitter where user_id = #{user_id} ")]
pub async fn delete_user_twitter(rb: &State<Arc<Rbatis>>,user_id: &str) -> Result<(),Error>{ }
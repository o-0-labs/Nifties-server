use std::sync::Arc;

use egg_mode::{Token, KeyPair, raw::{self, ParamList}};
use rbatis::{push_index, py_sql, rb_py, rbatis::Rbatis, Error};
use reqwest::{Response, Client};
use rocket::State;
use serde_json::Value;

use crate::{model::twitter_model::{UserTwitter, Tweets, AccessToken, Oauth, TimelineParams}, constant::{ACCESS_TOKEN, TWEET, CONSUMER_KEY, CONSUMER_SECRET, TIMELINE}};




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

pub async fn get_access_token(rb: &State<Arc<Rbatis>>, user_id: &str) -> Option<AccessToken>{
    match query_access_token(rb,user_id).await {
        Ok(r) => r,
        Err(e) => {
            error!("query_access_token error! {}",e);
            None
        },
    }
}


#[py_sql("select * from user_twitter where user_id= #{user_id} ")]
async fn query_user_twitter(rb: &State<Arc<Rbatis>>, user_id: &str) -> Option<UserTwitter> {
    todo!()
}

#[py_sql("insert into user_twitter (user_id,twitter_user_id,screen_name,access_token,access_token_secret) values (#{user_twitter.user_id},#{user_twitter.twitter_user_id},#{user_twitter.screen_name},#{user_twitter.access_token},#{user_twitter.access_token_secret}) ")]
async fn add_user_twitter(rb: &State<Arc<Rbatis>>, user_twitter: &UserTwitter) -> Result<(), Error> {
    todo!()
}

#[py_sql("update user_twitter set twitter_user_id = #{user_twitter.twitter_user_id}, screen_name = #{user_twitter.screen_name}, access_token = #{user_twitter.access_token}, access_token_secret = #{user_twitter.access_token_secret} where user_id = #{user_twitter.user_id} ")]
async fn update_user_twitter(rb: &State<Arc<Rbatis>>,user_twitter: &UserTwitter) -> Result<(),Error>{ }


#[py_sql("delete from user_twitter where user_id = #{user_id} ")]
async fn delete_user_twitter(rb: &State<Arc<Rbatis>>,user_id: &str) -> Result<(),Error>{ }


#[py_sql("select twitter_user_id,screen_name,access_token,access_token_secret from user_twitter where user_id= #{user_id} ")]
async fn query_access_token(rb: &State<Arc<Rbatis>>, user_id: &str) -> Option<AccessToken> {
    todo!()
}



pub async fn access(oauth: Oauth)->Result<Response, reqwest::Error>{

    let params = [("oauth_token",&oauth.oauth_token),("oauth_verifier",&oauth.oauth_verifier)];

    let client = Client::new();

    let res = client.post(ACCESS_TOKEN)
    .form(&params)
    .send()
    .await;

    res

}

pub async fn tweets(rb: &State<Arc<Rbatis>>, tweets: Tweets, user_id: &str)->Result<Value,String>{

    let con_token = KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

    let user_twitter = get_access_token(rb, user_id).await;

    if let Some(at) = user_twitter {
        let access_token = Token::Access {
            consumer: con_token,
            access: KeyPair::new(at.access_token,at.access_token_secret),
        };

        let req = raw::request_post_json(TWEET, &access_token, tweets);
        let res: Result<egg_mode::Response<serde_json::Value>, egg_mode::error::Error> = raw::response_json(req).await;
        match res {
            Ok(output) => {
                let json = output.response;
                info!("twitter api response ok! {}",json);
                let id = json["data"]["id"].clone();
                Ok(id)
            },
            Err(e) => {
                error!("twitter api response err! {}",e);
                Err(e.to_string())
            },
        }
    }else {
        error!("user has no access_token!");
        Err("user has no access_token!".to_string())
    }

}

pub async fn get_timeline(rb: &State<Arc<Rbatis>>, user_id: &str, timeline :TimelineParams) -> Result<Value,String>{

    let access_token = get_access_token(rb,user_id).await;

    if let Some(at) = access_token {
        let con_token = KeyPair::new(CONSUMER_KEY, CONSUMER_SECRET);

        let access_token = Token::Access {
            consumer: con_token,
            access: KeyPair::new(at.access_token,at.access_token_secret),
        };

        let timeline_url = TIMELINE.replace("twitter_user_id", &at.twitter_user_id);

        let mut params = ParamList::new();

        params = params.add_param("max_result", timeline.max_result.to_string());
        
        if let Some(s) = timeline.start_time {
            params = params.add_param("start_time", s);
        }

        if let Some(s) = timeline.end_time {
            params = params.add_param("end_time", s);
        }

        if let Some(s) = timeline.since_id {
            params = params.add_param("since_id", s);
        }

        if let Some(s) = timeline.until_id {
            params = params.add_param("until_id", s);
        }

        if let Some(s) = timeline.pagination_token {
            params = params.add_param("pagination_token", s);
        }

        if let Some(s) = timeline.tweet_fields {
            params = params.add_param("tweet.fields", s);
        }

        if let Some(s) = timeline.user_fields {
            params = params.add_param("user.fields", s);
        }

        if let Some(s) = timeline.media_fields {
            params = params.add_param("media.fields", s);
        }

        if let Some(s) = timeline.place_fields {
            params = params.add_param("place.fields", s);
        }

        if let Some(s) = timeline.poll_fields {
            params = params.add_param("poll.fields", s);
        }
     
        let req = raw::request_get(&timeline_url, &access_token, Some(&params));

        let res: Result<egg_mode::Response<serde_json::Value>, egg_mode::error::Error> = raw::response_json(req).await;

        match res {
            Ok(output) => {
                let json = output.response;
                info!("twitter api response ok! {}",json);
                Ok(json)
            },
            Err(e) => {
                error!("twitter api response err! {}",e);
                Err(e.to_string())
            },
        }
    }else{
        error!("user has no access_token!");
        Err("user has no access_token!".to_string())
    }
}
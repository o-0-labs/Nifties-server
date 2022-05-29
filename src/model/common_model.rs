use rbatis::CRUDTable;
use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::constant::KEY;

#[derive(CRUDTable, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user_id: String,
    pub pub_key: String,
    pub user_name:Option<String>,
    pub email:Option<String>,
    pub token: Option<String>,
    pub profile_photo: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
}


#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct UserAuth {
    pub sign_msg: String,
    pub pub_key: String,
    pub signature: String,

}


pub struct Token{
    pub sub: String,
}
// Bearer Token
impl Token {
    fn from_request(header: &str) -> Option<Token> {
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        if split_vec[0] != "Bearer" {
            return None;
        }
        Self::from_jwt(split_vec[1])
    }
    fn from_jwt(token_string: &str) -> Option<Token> {
        let val = Validation::default();
        //val.sub = Some("!Yg43#xQtBE357js".to_string());
        match decode::<Claims>(token_string, &DecodingKey::from_secret(KEY), &val) {
            Ok(c) => {
                println!("ExpTime:{:?}", c.claims.exp);
                let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
                if now > c.claims.exp{
                    return None;
                }
                return Some(Token{
                    sub:c.claims.sub
                });
            }
            Err(_) => None,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();
    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_request(header_auth) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}
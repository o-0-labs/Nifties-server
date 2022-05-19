use rocket::{fairing::{Fairing, Info, Kind}, Request, Response, http::Header};

//解决跨域
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// #[get("/hello")]
// async fn hello() -> String {
// //     ApiResponse::custom_error(MESSAGE_4000.to_string(),4000)
// "hello".to_string()
// }


 
#[catch(401)]
pub fn unvalid_token( ) -> String {
//     ApiResponse::custom_error(MESSAGE_4000.to_string(),4000)
"hello".to_string()
}
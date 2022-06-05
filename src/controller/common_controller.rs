use rocket::{fairing::{Fairing, Info, Kind}, Request, Response, http::{Header, ContentType}, fs::{NamedFile, TempFile}, response::content, form::Form};
use serde_json::{json, Value};
use std::path::{PathBuf, Path};
use uuid::Uuid;
use rocket_json_response::JSONResponse;

use crate::{constant::MAIN_URL, model::common_model::Token};




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


 
#[catch(401)]
pub fn unvalid_token( ) -> String {
//     ApiResponse::custom_error(MESSAGE_4000.to_string(),4000)
"unvalid_token".to_string()
}

#[catch(404)]
pub fn general_not_found() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
        <p>Hmm... What are you looking for?</p>
        Say <a href="/Niftes/index">hello!</a>
    "#)
}

#[get("/gettoken")]
pub fn twitter_token() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
        <p>Hmm... get token</p>
    "#)
}

#[get("/img/<path..>")]
pub async fn static_source(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new("img").join(path);
    if path.is_dir() {
        info!("{:?} path.is_dir",path);
        return None
    }
    info!("the img path is {:?}",path);
    NamedFile::open(path).await.ok()
}




#[derive(FromForm)]
pub struct Upload<'f> {
    pub upload: TempFile<'f>
}

#[post("/img/upload", data = "<form>")]
pub async fn upload(_auth: Token, mut form: Form<Upload<'_>>) -> JSONResponse<'static, Value> {

    info!("temp file path: {:#?}",form.upload);

    let suffix = check_content_type(form.upload.content_type());

    if let None = suffix {
        let msg = "Fail!";
        JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
    }else if let Some(s) = suffix {
        let mut file_name = String::from("img/");
        file_name.push_str(&Uuid::new_v4().to_string().replace("-", ""));
        file_name.push_str(&s);
        
        let save_path = Path::new(&file_name);
        match form.upload.persist_to(save_path).await{
            Ok(_) => {

                JSONResponse::ok(json!({"url": format!("{}{}", MAIN_URL, file_name) }))
            },
            Err(_) => {
            let msg = "Fail!";
            JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
            },
        }
    }else{
        let msg = "Fail!";
        JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
    }
   
}

fn check_content_type(content: Option<&ContentType>)->Option<String>{

    if let Some(s) = content {
        if s.is_jpeg() {
            return Some(".jpg".to_string())
        }else if s.is_bmp(){
            return Some(".bmp".to_string())
        }else if s.is_svg(){
            return Some(".svg".to_string())
        }else if s.is_png(){
            return Some(".png".to_string())
        }else if s.is_gif(){
            return Some(".gif".to_string())
        }else{
            return None
        }
    }
    None

}


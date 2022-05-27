use std::sync::Arc;

use rocket::{serde::json::{serde_json::json, Json, Value}, State};

use rbatis::{rbatis::Rbatis};
use rocket_json_response::{JSONResponse, JSONResponseWithoutData};

use crate::{model::test_model::{Article, ArticleQueryParams}, service::test_service::{select_service, insert_service, update_service, delete_service}};




#[post("/query", format = "json", data = "<params>")]
pub async fn query(params: Json<ArticleQueryParams>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    println!("{}",format!("{:?}", params));
    
    let re = select_service(rb,&params).await;

    match re {
        Ok(re) => JSONResponse::ok(json!(re)),
        Err(_) => {
            let msg = "Fail!";
            JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
        },
    }
}

#[post("/inssert", format = "json", data = "<article>")]
pub async fn insert(article: Json<Article>,rb: &State<Arc<Rbatis>>) -> JSONResponseWithoutData {
    println!("{}",format!("{:?}", article));
    

    let re = insert_service(rb,&article).await;

    match re {
        Ok(_) => JSONResponseWithoutData::ok(),
        Err(_) => JSONResponseWithoutData::err(1),
    }
}

#[post("/update", format = "json", data = "<article>")]
pub async fn update(article: Json<Article>,rb: &State<Arc<Rbatis>>) -> JSONResponseWithoutData{
    println!("{}",format!("{:?}", article));

    if article.id == None {
        return JSONResponseWithoutData::err(1)
    }

    let re = update_service(rb,&article).await;

    match re {
        Ok(_) => JSONResponseWithoutData::ok(),
        Err(_) => JSONResponseWithoutData::err(2),
    }
}

#[post("/delete", format = "json", data = "<article>")]
pub async fn delete(article: Json<Article>,rb: &State<Arc<Rbatis>>) -> JSONResponseWithoutData{
    println!("{}",format!("{:?}", article));

    if let None = article.id  {
        return JSONResponseWithoutData::err(1)
    }

    let re = delete_service(rb,&article).await;

    match re {
        Ok(_) => JSONResponseWithoutData::ok(),
        Err(_) => JSONResponseWithoutData::err(1),
    }

}



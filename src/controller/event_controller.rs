use std::sync::Arc;

use rocket::{serde::json::{serde_json::json, Json, Value}, State};
use rbatis::{rbatis::Rbatis};
use rocket_json_response::{JSONResponse};
use uuid::Uuid;

use crate::model::{common_model::{PageParams, Token}, event_model::Event};
use crate::service::event_service;
use crate::utils::util;


#[post("/event/query", format = "json", data = "<params>")]
pub async fn event_query(params: Json<PageParams>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("event/query parameter, {:?}",params);

    let p = params.into_inner();

    match event_service::event_query(rb,p).await{
        Ok(re) => {
            info!("event/query return ok");
            JSONResponse::ok(json!(re))
        },
        Err(e) => {
            error!("event/query return err, {}",e);
            JSONResponse::err(1,json!({"msg": "event query error!" }))
        },
    }
}


#[post("/event/add", format = "json", data = "<event>")]
pub async fn event_add(_auth: Token, event: Json<Event>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("event/add parameter, {:?}",event);

    let mut event = event.into_inner();

    match &event.user_id {
        Some(s) => {
            if s.trim().len() == 0 {
                let msg = "missing userid!";
                error!("event/add return err, {}",msg);
                return JSONResponse::err(1,json!({"msg": msg }))
            }

            if !_auth.sub.starts_with(s){
                let msg = "token Error!";
                error!("event/add return err, {}",msg);
                return JSONResponse::err(99,json!({"msg": msg }))
            }
        },
        None => {
            let msg = "missing userid!";
            error!("event/add return err, {}",msg);
            return JSONResponse::err(2,json!({"msg": msg }))
        },
    }

    if util::is_empty(&event.tag){
        let msg = "missing tag!";
        error!("event/add return err, {}",msg);
        return JSONResponse::err(3,json!({"msg": msg }))
    }

    if let Some(s) = &event.tag{
        if !s.eq("AMA") && !s.eq("MINT"){
            let msg = "tag error!";
            error!("event/add return err, {}",msg);
            return JSONResponse::err(3,json!({"msg": msg }))
        }
    }
    
    if util::is_empty(&event.title){
        let msg = "missing title!";
        error!("event/add return err, {}",msg);
        return JSONResponse::err(4,json!({"msg": msg }))
    }

    if util::is_empty(&event.description){
        let msg = "missing description!";
        error!("event/add return err, {}",msg);
        return JSONResponse::err(5,json!({"msg": msg }))
    }

    if util::is_empty(&event.image){
        let msg = "missing image!";
        error!("event/add return err, {}",msg);
        return JSONResponse::err(6,json!({"msg": msg }))
    }

    if util::is_empty(&event.user_name){
        let msg = "missing user_name!";
        error!("event/add return err, {}",msg);
        return JSONResponse::err(8,json!({"msg": msg }))
    }

    event.create_time = Some(rbatis::DateTimeNative::now());

    let event_id = Uuid::new_v4().to_string().replace("-", "");
    event.event_address = event_service::get_event_contract(rb,&event_id).await;

    if util::is_empty(&event.event_address){
        let msg = "there is no more event_address!";
        error!("event/add return err, {}",msg);
        return JSONResponse::err(7,json!({"msg": msg }))
    }

    event.event_id = Some(event_id);

    match event_service::event_add(rb,&event).await {
        Ok(_) => {
            info!("event/add return ok");
            JSONResponse::ok(json!({"msg": "success","event_address":event.event_address }))
        },
        Err(e) => {
            error!("event/add return err, {}",e);
            JSONResponse::err(10,json!({"msg": "add event error!" }))
        },
    }
}

#[post("/event/view", format = "json", data = "<event>")]
pub async fn event_view(event: Json<Event>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("event/view parameter, {:?}",event);

    let event = event.into_inner();

    if util::is_empty(&event.event_id){
        let msg = "missing event_id!";
        error!("event/view return err, {}",msg);
        return JSONResponse::err(1,json!({"msg": msg }))
    }

    match event_service::event_view(rb,&event).await {
        Ok(_) => {
            info!("event/view return ok");
            JSONResponse::ok(json!({"msg": "success" }))
        },
        Err(e) => {
            error!("event/view return err, {}",e);
            JSONResponse::err(2,json!({"msg": "event view error!" }))
        },
    }
}

#[post("/event/like", format = "json", data = "<event>")]
pub async fn event_like(event: Json<Event>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("event/like parameter, {:?}",event);

    let event = event.into_inner();

    if util::is_empty(&event.event_id){
        let msg = "missing event_id!";
        error!("event/like return err, {}",msg);
        return JSONResponse::err(1,json!({"msg": msg }))
    }

    match event_service::event_like(rb,&event).await {
        Ok(_) => {
            info!("event/like return ok");
            JSONResponse::ok(json!({"msg": "success" }))
        },
        Err(e) => {
            error!("event/like return err, {}",e);
            JSONResponse::err(2,json!({"msg": "event like error!" }))
        },
    }
}
use std::sync::Arc;

use rbatis::rbatis::Rbatis;
use rocket::{serde::json::{serde_json::json, Json, Value}, State};
use rocket_json_response::JSONResponse;
use uuid::Uuid;

use crate::service::grants_service;
use crate::model::{common_model::{PageParams, Token}, grants_model::Grants};
use crate::utils::util;


#[post("/grants/query", format = "json", data = "<params>")]
pub async fn grants_query(params: Json<PageParams>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("grants/query parameter, {:?}",params);

    let p = params.into_inner();

    match grants_service::grants_query(rb,p).await{
        Ok(re) => {
            info!("grants/query return ok");
            JSONResponse::ok(json!(re))
        },
        Err(e) => {
            error!("grants/query return err, {}",e);
            JSONResponse::err(1,json!({"msg": "query Fail!" }))
        },
    }
}

#[post("/grants/add", format = "json", data = "<grants>")]
pub async fn grants_add(_auth: Token, grants: Json<Grants>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("grants/add parameter, {:?}",grants);

    let mut grants = grants.into_inner();

    match &grants.user_id {
        Some(s) => {
            if s.trim().len() == 0 {
                let msg = "missing userid!";
                error!("grants/add return err, {}",msg);
                return JSONResponse::err(1,json!({"msg": msg }))
            }

            if !_auth.sub.starts_with(s){
                let msg = "token Error!";
                error!("grants/add return err, {}",msg);
                return JSONResponse::err(99,json!({"msg": msg }))
            }
        },
        None => {
            let msg = "missing userid!";
            error!("grants/add return err, {}",msg);
            return JSONResponse::err(2,json!({"msg": msg }))
        },
    }

    if util::is_empty(&grants.title){
        let msg = "missing title!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(3,json!({"msg": msg }))
    }

    if util::is_empty(&grants.user_name){
        let msg = "missing user_name!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(4,json!({"msg": msg }))
    }

    if util::is_empty(&grants.description){
        let msg = "missing description!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(5,json!({"msg": msg }))
    }

    if util::is_empty(&grants.logo){
        let msg = "missing logo!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(9,json!({"msg": msg }))
    }

    grants.contract_address = grants_service::get_grants_contract(rb).await;

    if util::is_empty(&grants.contract_address){
        let msg = "there is no contract_address!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(9,json!({"msg": msg }))
    }
    if util::is_empty(&grants.website){
        let msg = "missing website!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(9,json!({"msg": msg }))
    }
    if util::is_empty(&grants.twitter){
        let msg = "missing twitter!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(9,json!({"msg": msg }))
    }
    if util::is_empty(&grants.bringing){
        let msg = "missing bringing!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(9,json!({"msg": msg }))
    }
    if util::is_empty(&grants.external_funding){
        let msg = "missing external_funding!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(9,json!({"msg": msg }))
    }
    if util::is_empty(&grants.based){
        let msg = "missing based!";
        error!("grants/add return err, {}",msg);
        return JSONResponse::err(9,json!({"msg": msg }))
    }


    grants.create_time = Some(rbatis::DateTimeNative::now());
    grants.grants_id = Some(Uuid::new_v4().to_string().replace("-", ""));

    match grants_service::grants_add(rb,grants).await {
        Ok(_) => {
            info!("grants/add return ok");
            JSONResponse::ok(json!({"msg": "success" }))
        },
        Err(e) => {
            error!("grants/add return err, {}",e);
            JSONResponse::err(10,json!({"msg": "add grants error!" }))
        },
    }
}
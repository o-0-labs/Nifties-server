use std::sync::Arc;

use rbatis::rbatis::Rbatis;
use rocket::{serde::json::{serde_json::json, Json, Value}, State};
use rocket_json_response::JSONResponse;

use crate::service::hackathon_service;
use crate::model::common_model::{PageParams, Token};
use crate::model::hackathon_model::{UserHackathon, Hackathon};
use crate::utils::util;

#[post("/hackathon/count")]
pub async fn hackathon_count(rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("hackathon/count parameter, ..");

    match hackathon_service::hackathon_count(rb).await{
        Ok(re) => {
            info!("hackathon/count return ok");
            JSONResponse::ok(json!(re))
        },
        Err(e) => {
            error!("hackathon/count return err, {}",e);
            JSONResponse::err(1,json!({"msg": "hackathon count error!" }))
        },
    }
}

#[post("/hackathon/query", format = "json", data = "<params>")]
pub async fn hackathon_query(params: Json<PageParams>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("hackathon/query parameter, {:?}",params);

    let p = params.into_inner();

    match hackathon_service::hackathon_query(rb,p).await{
        Ok(re) => {
            info!("hackathon/query return ok");
            JSONResponse::ok(json!(re))
        },
        Err(e) => {
            error!("hackathon/query return err, {}",e);
            JSONResponse::err(1,json!({"msg": "hackathon query error!" }))
        },
    }
}

#[post("/hackathon/querybyuser", format = "json", data = "<params>")]
pub async fn hackathon_query_by_user(_auth: Token,params: Json<PageParams>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("hackathon/query parameter, {:?}",params);

    let p = params.into_inner();

    let user_id = &_auth.sub[0..32];

    match hackathon_service::hackathon_query_by_user(rb, p,user_id).await {
        Ok(re) => {
            info!("hackathon/query return ok");
            JSONResponse::ok(json!(re))
        },
        Err(e) => {
            error!("hackathon/query return err, {}",e);
            JSONResponse::err(1,json!({"msg": "hackathon query error!" }))
        },
    }
}

#[post("/hackathon/join", format = "json", data = "<hackathon>")]
pub async fn hackathon_join(_auth: Token, hackathon: Json<UserHackathon>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("hackathon/join parameter, {:?}",hackathon);

    let mut hackathon = hackathon.into_inner();


    match &hackathon.user_id {
        Some(s) => {
            if s.trim().len() == 0 {
                let msg = "missing userid!";
                error!("hackathon/join return err, {}",msg);
                return JSONResponse::err(1,json!({"msg": msg }))
            }

            if !_auth.sub.starts_with(s){
                let msg = "token Error!";
                error!("hackathon/join return err, {}",msg);
                return JSONResponse::err(99,json!({"msg": msg }))
            }
        },
        None => {
            let msg = "missing userid!";
            error!("hackathon/join return err, {}",msg);
            return JSONResponse::err(2,json!({"msg": msg }))
        },
    }

    if let Some(user_id) = &hackathon.user_id {
        match &hackathon.hackathon_id {
            Some(hackathon_id) => {
                if hackathon_id.trim().len() == 0 {
                    let msg = "missing hackathon_id!";
                    error!("hackathon/join return err, {}",msg);
                    return JSONResponse::err(3,json!({"msg": msg }))
                }
                
                if let Err(e) = hackathon_service::hackathon_join_check(rb, hackathon_id, user_id).await{
                    error!("hackathon/join return err, {}",e);
                    return JSONResponse::err(4,json!({"msg": e }))
                }
            },
            None => {
                let msg = "missing hackathon_id!";
                error!("hackathon/join return err, {}",msg);
                return JSONResponse::err(3,json!({"msg": msg }))
            },
        }
    }
    

    if util::is_empty(&hackathon.discord){
        let msg = "missing discord!";
        error!("hackathon/join return err, {}",msg);
        return JSONResponse::err(3,json!({"msg": msg }))
    }

    if util::is_empty(&hackathon.sharing_email){
        let msg = "missing sharing_email!";
        error!("hackathon/join return err, {}",msg);
        return JSONResponse::err(3,json!({"msg": msg }))
    }

    if let Some(s) = &hackathon.sharing_email{
        if !s.eq("0") && !s.eq("1"){
            let msg = "sharing_email error!";
            error!("hackathon/join return err, {}",msg);
            return JSONResponse::err(3,json!({"msg": msg }))
        }
    }

    if util::is_empty(&hackathon.agree){
        let msg = "missing agree!";
        error!("hackathon/join return err, {}",msg);
        return JSONResponse::err(3,json!({"msg": msg }))
    }

    if let Some(s) = &hackathon.agree{
        if !s.eq("0") && !s.eq("1"){
            let msg = "agree error!";
            error!("hackathon/join return err, {}",msg);
            return JSONResponse::err(3,json!({"msg": msg }))
        }
    }

    hackathon.join_time = Some(rbatis::DateTimeNative::now());

    match hackathon_service::hackathon_join(rb,&hackathon).await {
        Ok(_) => {
            info!("hackathon/join return ok");
            JSONResponse::ok(json!({"msg": "success" }))
        },
        Err(e) => {
            error!("hackathon/join return err, {}",e);
            JSONResponse::err(10,json!({"msg": "hackathon join error!" }))
        },
    }
}


#[post("/hackathon/detail", format = "json", data = "<hackathon>")]
pub async fn hackathon_detail(hackathon: Json<Hackathon>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("hackathon/detail parameter, {:?}",hackathon);

    let h = hackathon.into_inner();

    match h.hackathon_id {
        Some(h) => {
            if h.trim().len() == 0 {
                let msg = "missing hackathon_id!";
                error!("hackathon/detail return err, {}",msg);
                return JSONResponse::err(1,json!({"msg": msg }))
            }else{
                match hackathon_service::query_detail(rb, &h).await {
                    Ok(re) => {
                        info!("hackathon/detail return ok");
                        JSONResponse::ok(json!(re))
                    },
                    Err(e) => {
                        error!("hackathon/detail return err, {}",e);
                        JSONResponse::err(1,json!({"msg": e }))
                    },
                }
            }
        },
        None => {
            let msg = "missing hackathon_id!";
            error!("hackathon/detail return err, {}",msg);
            return JSONResponse::err(2,json!({"msg": msg }))
        },
    }    
}

#[post("/hackathon/checkjoin", format = "json", data = "<hackathon>")]
pub async fn check_join(_auth: Token,hackathon: Json<Hackathon>,rb: &State<Arc<Rbatis>>) -> JSONResponse<'static, Value> {
    info!("hackathon/checkjoin parameter, {:?}",hackathon);

    let hackathon = hackathon.into_inner();

    let user_id = &_auth.sub[0..32];


    if let Some(s) = hackathon.hackathon_id {
        match hackathon_service::hackathon_join_check(rb,&s,user_id).await{
            Ok(_) => {
                JSONResponse::ok(json!({"join_flag": "1" }))
            },
            Err(s) => {
                if s.eq("already joined") {
                    JSONResponse::ok(json!({"join_flag": "0" }))
                }else{
                    JSONResponse::err(2,json!({"msg": s }))
                }
            },
        }
        
    }else{
        let msg = "missing hackathon_id!";
        error!("hackathon/checkjoin return err, {}",msg);
        return JSONResponse::err(1,json!({"msg": msg }))
    }

}
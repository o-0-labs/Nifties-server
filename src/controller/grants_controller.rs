use std::sync::Arc;

use rbatis::rbatis::Rbatis;
use rocket::{serde::json::{serde_json::json, Json, Value}, State};
use rocket_json_response::JSONResponse;

use crate::service::grants_service;
use crate::model::common_model::PageParams;


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
            let msg = "query Fail!";
            error!("grants/query return err, {}",e);
            JSONResponse::err(1,json!({"msg": format!("{}", msg) }))
        },
    }
}
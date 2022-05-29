use std::sync::Arc;

use rbatis::{rbatis::Rbatis, Error, py_sql, rb_py, push_index, PageRequest, Page};
use rocket::State;

use crate::model::{common_model::PageParams,grants_model::Grants};


pub async fn grants_query(rb: &State<Arc<Rbatis>>, params: PageParams) -> Result<Page<Grants>,Error>{
    let page_req = &PageRequest::new(params.page_no,params.page_size);
    query_grants(rb,page_req,&params).await
}

#[py_sql("select * from grants where delete_flag='0'
if params.status != null && params.status != '': 
and status = #{params.status} ")]
async fn query_grants(rb: &State<Arc<Rbatis>>, page_req: &PageRequest, params: &PageParams) -> Page<Grants> { todo!() }
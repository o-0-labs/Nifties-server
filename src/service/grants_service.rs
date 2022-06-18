use std::sync::Arc;

use rbatis::{rbatis::Rbatis, Error, py_sql, rb_py, push_index, PageRequest, Page};
use rocket::State;

use crate::model::{common_model::PageParams,grants_model::{Grants, GrantsAddress}};


pub async fn grants_query(rb: &State<Arc<Rbatis>>, params: PageParams) -> Result<Page<Grants>,Error>{
    let page_req = &PageRequest::new(params.page_no,params.page_size);
    query_grants(rb,page_req,&params).await
}

pub async fn grants_add(rb: &State<Arc<Rbatis>>, grants: Grants) -> Result<(),Error>{
    insert_grants(rb,&grants).await
}

#[py_sql("select * from grants where delete_flag='0'
if params.status != null && params.status != '': 
    and status = #{params.status} ")]
async fn query_grants(rb: &State<Arc<Rbatis>>, page_req: &PageRequest, params: &PageParams) -> Page<Grants> { todo!() }

#[py_sql("insert into grants 
(grants_id,title,user_id,user_name,description,total_raised,logo,contract_address,
website,twitter,bringing,external_funding,based,create_time)
values 
(#{grants.grants_id},#{grants.title},#{grants.user_id},#{grants.user_name},
#{grants.description},#{grants.total_raised},#{grants.logo},#{grants.contract_address},
#{grants.website},#{grants.twitter},#{grants.bringing},#{grants.external_funding},
#{grants.based},#{grants.create_time}) 
")]
async fn insert_grants(rb: &State<Arc<Rbatis>>, grants: &Grants) -> Result<(),Error>{ todo!() }

pub async fn get_grants_contract(rb: &State<Arc<Rbatis>>) -> Option<String>{
    match get_contract(rb).await{
        Ok(r) => {
            match r{
                Some(s) => {
                    Some(s.contract_address)
                },
                None => None,
            }
        },
        Err(e) => {
            error!("get_grants_contract error! {}",e);
            None
        },
    }
}

#[py_sql("select * from grants_address where grants_id is null limit 1 ")]
async fn get_contract(rb: &State<Arc<Rbatis>>) -> Option<GrantsAddress>{
    todo!()
}
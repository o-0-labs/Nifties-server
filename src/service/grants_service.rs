use std::sync::Arc;

use rbatis::{rbatis::Rbatis, Error, py_sql, rb_py, push_index, PageRequest, Page, db::DBExecResult};
use rocket::State;

use crate::model::{common_model::PageParams,grants_model::{Grants, GrantsAddress}};


pub async fn grants_query(rb: &State<Arc<Rbatis>>, params: PageParams) -> Result<Page<Grants>,Error>{
    let page_req = &PageRequest::new(params.page_no,params.page_size);
    query_grants(rb,page_req,&params).await
}

pub async fn grants_add(rb: &State<Arc<Rbatis>>, grants: &Grants) -> Result<(),Error>{
    insert_grants(rb,grants).await
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

pub async fn get_grants_contract(rb: &State<Arc<Rbatis>>,grants_id: &str) -> Option<String>{
    match get_contract(rb).await{
        Ok(r) => {
            match r{
                Some(s) => {
                    match contract_add_grants_id(rb,grants_id,&s.contract_address).await{
                        Ok(dbresult) => {
                            if dbresult.rows_affected == 1{
                                Some(s.contract_address)
                            }else{
                                error!("contract_add_grants_id rows_affected:0 !");
                                None
                            }
                        },
                        Err(e) => {
                            error!("contract_add_grants_id error! {}",e);
                            None
                        },
                    }
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

#[py_sql("update grants_address set grants_id=#{grants_id} where grants_id is null and contract_address=#{contract_address} ")]
async fn contract_add_grants_id(rb: &State<Arc<Rbatis>>,grants_id: &str,contract_address: &str) -> Result<DBExecResult,Error>{
    todo!()
}

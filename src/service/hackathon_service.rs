use std::sync::Arc;

use rbatis::{rbatis::Rbatis, Error, py_sql, rb_py, push_index, PageRequest, Page};
use rocket::State;

use crate::{model::{common_model::PageParams,hackathon_model::{HackathonCount, Hackathon, UserHackathon}}};

pub async fn hackathon_count(rb: &State<Arc<Rbatis>>) -> Result<Option<HackathonCount>,Error>{
    query_hackathon_count(rb).await
}

pub async fn hackathon_query(rb: &State<Arc<Rbatis>>, params: PageParams) -> Result<Page<Hackathon>,Error>{
    let page_req = &PageRequest::new(params.page_no,params.page_size);
    query_hackathon(rb,page_req,&params).await
}

pub async fn hackathon_join_check<'a,'b>(rb: &'a State<Arc<Rbatis>>,hackathon_id: &'b str, user_id: &'b str) -> Result<(),&'b str>{
    
    match query_hackathon_by_id(rb,hackathon_id).await {
        Ok(h) => match h {
            Some(_) => {
                match query_user_hackathon_by_id(rb,hackathon_id,user_id).await{
                    Ok(uh) => {
                        match uh {
                            Some(_) => Err("already joined"),
                            None => Ok(()),
                        }
                    },
                    Err(e) => {
                        error!("query_user_hackathon_by_id, {}",e);
                        Err("query error")
                    },
                }
                
            },
            None => Err("hackathon_id error or hackathon is not happening"),
        },
        Err(e) => {
            error!("query_hackathon_by_id, {}",e);
            Err("query error")
        },
    }
}

pub async fn hackathon_join(rb: &State<Arc<Rbatis>>,hackathon: &UserHackathon) -> Result<(),Error>{
    user_hackathon_insert(rb,&hackathon).await
}


#[py_sql("select (select count(1) from hackathon where status = '1' and delete_flag = '0') as happening,
(select count(1) from hackathon where status = '2' and delete_flag = '0') as upcoming,
(select count(1) from hackathon where status = '2' and delete_flag = '0') as completed from dual ")]
async fn query_hackathon_count(rb: &State<Arc<Rbatis>>) -> Option<HackathonCount> { todo!() }


#[py_sql("select * from hackathon where delete_flag='0'
if params.status != null && params.status != '': 
    and status = #{params.status} ")]
async fn query_hackathon(rb: &State<Arc<Rbatis>>, page_req: &PageRequest, params: &PageParams) -> Page<Hackathon> { todo!() }


#[py_sql("select * from hackathon where delete_flag='0' and status='1' and hackathon_id=#{hackathon_id} ")]
async fn query_hackathon_by_id(rb: &State<Arc<Rbatis>>,hackathon_id: &str) -> Option<Hackathon> { todo!() }

#[py_sql("select * from user_hackathon where  hackathon_id=#{hackathon_id} and user_id=#{user_id}")]
async fn query_user_hackathon_by_id(rb: &State<Arc<Rbatis>>,hackathon_id: &str,user_id: &str) -> Option<UserHackathon> { todo!() }


#[py_sql("insert into user_hackathon 
(hackathon_id,user_id,discord,sharing_email,agree,join_time)
values
(#{hackathon.hackathon_id},#{hackathon.user_id},#{hackathon.discord},#{hackathon.sharing_email},#{hackathon.agree},#{hackathon.join_time})
")]
async fn user_hackathon_insert(rb: &State<Arc<Rbatis>>,hackathon: &UserHackathon) -> Result<(),Error>{  todo!() }

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

pub async fn hackathon_query_by_user(rb: &State<Arc<Rbatis>>, params: PageParams, user_id: &str) -> Result<Page<Hackathon>,Error>{
    let page_req = &PageRequest::new(params.page_no,params.page_size);
    query_hackathon_by_user(rb,page_req,&params,user_id).await
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

pub async fn query_detail(rb: &State<Arc<Rbatis>>,hackathon_id: &str)->Result<Hackathon,String>{

    match query_hackathon_by_id(rb,hackathon_id).await {
        Ok(r) => {
            match r {
                Some(h) => Ok(h),
                None => {
                    error!("no detail,hackathon_id error!");
                    Err("no detail,hackathon_id error!".to_string())
                },
            }
        },
        Err(e) => {
            error!("query hackathon detail error!{}",e);
            Err("query hackathon detail error!".to_string())
        },
    }


}


#[py_sql("select (select count(1) from hackathon where status = '1' and delete_flag = '0') as happening,
(select count(1) from hackathon where status = '2' and delete_flag = '0') as upcoming,
(select count(1) from hackathon where status = '2' and delete_flag = '0') as completed from dual ")]
async fn query_hackathon_count(rb: &State<Arc<Rbatis>>) -> Option<HackathonCount> { todo!() }


#[py_sql("select hackathon_id,title,date,description,sponsored,status,image,discord_url from hackathon where delete_flag='0'
if params.status != null && params.status != '': 
    and status = #{params.status} ")]
async fn query_hackathon(rb: &State<Arc<Rbatis>>, page_req: &PageRequest, params: &PageParams) -> Page<Hackathon> { todo!() }

#[py_sql("select a.hackathon_id as hackathon_id,a.title as title,
a.date as date,a.description as description,a.sponsored as sponsored,
a.status as status,a.image as image,a.discord_url as discord_url,
case when b.agree='1' then '1' else '0' end as join_flag  
from hackathon a left join (select * from user_hackathon where user_id = #{user_id}) b 
on a.hackathon_id=b.hackathon_id where a.delete_flag='0'
if params.status != null && params.status != '': 
    and a.status = #{params.status} ")]
async fn query_hackathon_by_user(rb: &State<Arc<Rbatis>>, page_req: &PageRequest, params: &PageParams, user_id: &str) -> Page<Hackathon> { todo!() }


#[py_sql("select * from hackathon where delete_flag='0' and hackathon_id=#{hackathon_id} ")]
async fn query_hackathon_by_id(rb: &State<Arc<Rbatis>>,hackathon_id: &str) -> Option<Hackathon> { todo!() }

#[py_sql("select * from user_hackathon where  hackathon_id=#{hackathon_id} and user_id=#{user_id}")]
async fn query_user_hackathon_by_id(rb: &State<Arc<Rbatis>>,hackathon_id: &str,user_id: &str) -> Option<UserHackathon> { todo!() }


#[py_sql("insert into user_hackathon 
(hackathon_id,user_id,discord,sharing_email,agree,join_time)
values
(#{hackathon.hackathon_id},#{hackathon.user_id},#{hackathon.discord},#{hackathon.sharing_email},#{hackathon.agree},#{hackathon.join_time})
")]
async fn user_hackathon_insert(rb: &State<Arc<Rbatis>>,hackathon: &UserHackathon) -> Result<(),Error>{  todo!() }

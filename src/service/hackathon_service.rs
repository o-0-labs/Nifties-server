use std::sync::Arc;

use rbatis::{rbatis::Rbatis, Error, py_sql, rb_py, push_index, PageRequest, Page};
use rocket::State;

use crate::model::{common_model::PageParams,hackathon_model::{HackathonCount, Hackathon, UserHackathon}};

pub async fn hackathon_count(rb: &State<Arc<Rbatis>>) -> Result<Option<HackathonCount>,Error>{
    query_hackathon_count(rb).await
}

pub async fn hackathon_query(rb: &State<Arc<Rbatis>>, params: PageParams) -> Result<Page<Hackathon>,Error>{
    let page_req = &PageRequest::new(params.page_no,params.page_size);
    query_hackathon(rb,page_req,&params).await
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

#[py_sql("insert into user_hackathon 
(hackathon_id,user_id,discord,sharing_email,agree,join_time)
values
(#{hackathon.hackathon_id},#{hackathon.user_id},#{hackathon.discord},#{hackathon.sharing_email},#{hackathon.agree},#{hackathon.join_time})
")]
async fn user_hackathon_insert(rb: &State<Arc<Rbatis>>,hackathon: &UserHackathon) -> Result<(),Error>{  todo!() }

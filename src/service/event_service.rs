use std::sync::Arc;

use rbatis::{rbatis::Rbatis, Error, py_sql, rb_py, push_index, PageRequest, Page};
use rocket::State;

use crate::model::{common_model::PageParams, event_model::Event};




pub async fn event_query(rb: &State<Arc<Rbatis>>, params: PageParams) -> Result<Page<Event>,Error>{
    let page_req = &PageRequest::new(params.page_no,params.page_size);
    
    event_select(rb,page_req,&params).await
}

pub async fn event_add(rb: &State<Arc<Rbatis>>,event: &Event) -> Result<(),Error>{
    event_insert(rb,&event).await
}

pub async fn event_view(rb: &State<Arc<Rbatis>>,event: &Event) -> Result<(),Error>{
    event_view_update(rb,&event).await
}

pub async fn event_like(rb: &State<Arc<Rbatis>>,event: &Event) -> Result<(),Error>{
    event_like_update(rb,&event).await
}


#[py_sql("select * from event where delete_flag = '0'  
if params.status != null && params.status != '': 
    and status = #{params.status} ")]
async fn event_select(rb: &State<Arc<Rbatis>>, page_req: &PageRequest, params: &PageParams) -> Page<Event> { todo!() }


#[py_sql("insert into event 
(event_id,title,description,image,event_address,user_id,user_name,create_time,tag)
values
(#{event.event_id},#{event.title},#{event.description},#{event.image},#{event.event_address},
#{event.user_id},#{event.user_name},#{event.create_time},#{event.tag})
")]
pub async fn event_insert(rb: &State<Arc<Rbatis>>,event: &Event) -> Result<(),Error>{  todo!() }

#[py_sql("update event set
event_view = event_view + 1 
where event_id = #{event.event_id}
")]
async fn event_view_update(rb: &State<Arc<Rbatis>>,event: &Event) -> Result<(),Error>{  todo!() }

#[py_sql("update event set
event_like = event_like + 1 
where event_id = #{event.event_id}
")]
async fn event_like_update(rb: &State<Arc<Rbatis>>,event: &Event) -> Result<(),Error>{  todo!() }
use std::sync::Arc;

use rbatis::{rbatis::Rbatis, Error, py_sql, rb_py, push_index, PageRequest, Page};
use rocket::State;

use crate::model::test_model::{Article, ArticleQueryParams};


pub async fn insert_service(rb: &State<Arc<Rbatis>>,article: &Article) -> Result<(),Error>{

    let article = Article {
        create_time : Some(rbatis::DateTimeNative::now()),
        author : article.author.clone(),
        content : article.content.clone(),
        title : article.title.clone(),
        id : None,
    };

    py_insert(rb,&article).await
    
}

pub async fn select_service(rb: &State<Arc<Rbatis>>, params: &ArticleQueryParams) -> Result<Page<Article>,Error>{
    let page_req = &PageRequest::new(params.page_no,params.page_size);
    
    py_select(rb,page_req,params).await
}

pub async fn update_service(rb: &State<Arc<Rbatis>>,article: &Article) -> Result<(),Error>{
    
        py_update(rb,article).await
}

pub async fn delete_service(rb: &State<Arc<Rbatis>>,article: &Article) -> Result<(),Error>{
    py_delete(rb,article).await
}


#[py_sql("insert into article 
(title,author,content,create_time)
values
(#{article.title},#{article.author},#{article.content},#{article.create_time})
")]
pub async fn py_insert(rb: &State<Arc<Rbatis>>,article: &Article) -> Result<(),Error>{  todo!() }


#[py_sql("select * from article where delete_flag = '0'  
if params.title != null && params.title != '': 
    and title = #{params.title} 
if params.content != null && params.content != '': 
    and content = #{params.author} 
if params.author != null && params.author != '': 
    and author = #{params.author} ")]
pub async fn py_select(rb: &State<Arc<Rbatis>>, page_req: &PageRequest, params: &ArticleQueryParams) -> Page<Article> { todo!() }

#[py_sql("update article set title = #{article.title}, author = #{article.author}, content = #{article.content} where id = #{article.id} ")]
pub async fn py_update(rb: &State<Arc<Rbatis>>,article: &Article) -> Result<(),Error>{ }

// #[py_sql("delete from article where id = #{article.id} ")]
#[py_sql("update article set delete_flag='1' where id = #{article.id} ")]
pub async fn py_delete(rb: &State<Arc<Rbatis>>,article: &Article) -> Result<(),Error>{ }
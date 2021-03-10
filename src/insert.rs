//! 用于插入和更新
use crate::schema::*;

#[derive(Insertable, AsChangeset)]
#[table_name = "histories"]
pub struct InsertHistory {
    pub post_id: u32,
    pub time: u32,
    pub markdown: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "post_edge"]
pub struct InsertEdge {
    pub from_post: u32,
    pub to_post: u32,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "posts"]
pub struct InsertPost {
    pub title: String,
    pub markdown: Option<String>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "tokens"]
pub struct InsertToken {
    pub user_id: u32,
    pub token: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "users"]
pub struct InsertUser {
    pub nickname: String,
    pub email: String,
    pub password: String,
}

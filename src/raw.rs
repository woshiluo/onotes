//! 用于读取数据库
use crate::schema::*;

#[derive(Queryable, Insertable)]
#[table_name = "histories"]
pub struct RawHistory {
    pub id: u32,
    pub post_id: u32,
    pub time: u32,
    pub markdown: Option<String>,
}

#[derive(Queryable, Insertable)]
#[table_name = "post_edge"]
pub struct RawEdge {
    pub id: u32,
    pub from_post: u32,
    pub to_post: u32,
}

#[derive(Queryable, Insertable)]
#[table_name = "posts"]
pub struct RawPost {
    pub id: u32,
    pub title: String,
    pub markdown: Option<String>,
}

#[derive(Queryable, Insertable)]
#[table_name = "tokens"]
pub struct RawToken {
    pub id: u32,
    pub user_id: u32,
    pub token: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct RawUser {
    pub id: u32,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub admin: bool,
}

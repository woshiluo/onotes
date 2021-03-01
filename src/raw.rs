//! 用于读取数据库
use crate::schema::*;

#[derive(Queryable, Insertable)]
#[table_name = "histories"]
pub struct RawHistory {
    pub id: Option<i32>,
    pub post_id: i32,
    pub time: i32,
    pub markdown: Option<String>,
}

#[derive(Queryable, Insertable)]
#[table_name = "post_edge"]
pub struct RawEdge {
    pub id: Option<i32>,
    pub from_post: i32,
    pub to_post: i32,
}

#[derive(Queryable, Insertable)]
#[table_name = "posts"]
pub struct RawPost {
    pub id: Option<i32>,
    pub title: String,
    pub markdown: Option<String>,
}

#[derive(Queryable, Insertable)]
#[table_name = "tokens"]
pub struct RawToken {
    pub id: Option<i32>,
    pub user_id: i32,
    pub token: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "users"]
pub struct RawUser {
    pub id: Option<i32>,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub admin: i32,
}

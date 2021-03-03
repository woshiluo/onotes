#[macro_use]
extern crate diesel;

pub mod insert;
pub mod raw;
pub mod schema;

pub mod auth;
pub mod edge;
pub mod history;
pub mod post;
pub mod token;
pub mod user;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde::{Deserialize, Serialize};

const TOKEN_LEN: u32 = 32;

type DbConn = diesel::SqliteConnection;

no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

// TODO: 进一步展开 SQLError
/// Note 错误类型
#[derive(Debug, Serialize, Deserialize)]
pub enum NoteError {
    /// 无法找到用户
    UserNotFound(String),
    /// 认证失败
    AuthError(String),
    /// 没有权限
    NoPermission(String),
    /// SQL 错误
    SQLError(String),
}

/// 生成一个长度为 `token_len` 的随机字符串，作为 Token
pub fn gen_token() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::iter;

    let mut rng = thread_rng();
    iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(TOKEN_LEN as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

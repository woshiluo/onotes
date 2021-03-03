//! 历史记录
use crate::auth::{AuthDelete, AuthInsert, AuthUser};
use crate::raw::RawHistory;
use crate::DbConn;
use crate::NoteError;

use crate::insert::InsertHistory;

/// 文章的历史记录
pub struct History {
    id: i32,
    post_id: i32,
    /// 文章 id
    time: i32,
    /// 这次历史记录的时间
    markdown: Option<String>,
}

impl History {
    pub fn new(post_id: i32, post_mardown: &str) -> History {
        History {
            id: 0,
            post_id,
            time: chrono::Utc::now().timestamp() as i32,
            markdown: Some(String::from(post_mardown)),
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_post_id(&self) -> i32 {
        self.post_id
    }
    pub fn get_time(&self) -> i32 {
        self.time
    }
    pub fn get_markdown(&self) -> &str {
        match &self.markdown {
            Some(markdown) => markdown,
            None => "",
        }
    }

    /// 获取某篇文章的历史记录列表
    pub fn get_history(query_id: i32, conn: &DbConn) -> Result<Vec<History>, NoteError> {
        use crate::diesel::*;
        use crate::schema::histories::dsl::*;
        let history_list = histories
            .filter(post_id.eq(query_id))
            .load::<RawHistory>(conn)
            .map_err(|err| {
                NoteError::SQLError(format!("Failed query history of {}: {}", query_id, err))
            })?
            .iter()
            .map(History::from)
            .collect::<Vec<History>>();

        Ok(history_list)
    }
}

impl AuthInsert for History {
    fn insert(&self, conn: &DbConn, user: &AuthUser) -> Result<i32, NoteError> {
        use crate::diesel::*;
        use crate::schema::histories::dsl::*;

        user.auth()?;

        diesel::insert_into(histories)
            .values(InsertHistory::from(&*self))
            .execute(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to insert history: {}", err)))?;

        let return_id = histories
            .select(crate::last_insert_rowid)
            .get_result::<i32>(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to query insert id: {}", err)))?;
        Ok(return_id)
    }
}

impl AuthDelete for History {
    fn delete(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::histories::dsl::*;

        user.auth()?;

        diesel::delete(histories.filter(id.eq(self.get_id())))
            .execute(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to delete history: {}", err)))?;

        Ok(())
    }
}

impl From<&RawHistory> for History {
    fn from(history: &RawHistory) -> History {
        History {
            id: history.id.expect("history id is null!"),
            post_id: history.post_id,
            markdown: history.markdown.clone(),
            time: history.time,
        }
    }
}

impl From<&History> for InsertHistory {
    fn from(history: &History) -> InsertHistory {
        InsertHistory {
            post_id: history.get_post_id(),
            time: history.get_time(),
            markdown: Some(String::from(history.get_markdown())),
        }
    }
}

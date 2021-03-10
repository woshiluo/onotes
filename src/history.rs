//! 历史记录
use crate::auth::{AuthDelete, AuthInsert, AuthUser};
use crate::raw::RawHistory;
use crate::DbConn;
use crate::NoteError;

use crate::insert::InsertHistory;

/// 文章的历史记录
#[derive(Serialize, Deserialize)]
pub struct History {
    id: u32,
    post_id: u32,
    /// 文章 id
    time: u32,
    /// 这次历史记录的时间
    markdown: Option<String>,
}

impl History {
    pub fn new(post_id: u32, post_mardown: &str) -> History {
        History {
            id: 0,
            post_id,
            time: chrono::Utc::now().timestamp() as u32,
            markdown: Some(String::from(post_mardown)),
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_post_id(&self) -> u32 {
        self.post_id
    }
    pub fn get_time(&self) -> u32 {
        self.time
    }
    pub fn get_markdown(&self) -> &str {
        match &self.markdown {
            Some(markdown) => markdown,
            None => "",
        }
    }

    pub fn from_id(conn: &DbConn, query_id: u32) -> Result<History, NoteError> {
        use crate::diesel::*;
        use crate::schema::histories::dsl::*;

        let history = History::from(
            &histories
                .filter(id.eq(query_id))
                .first::<RawHistory>(conn)
                .map_err(|err| {
                    NoteError::SQLError(format!("Failed query history of {}: {}", query_id, err))
                })?,
        );

        Ok(history)
    }

    /// 获取某篇文章的历史记录列表
    pub fn get_history(query_id: u32, conn: &DbConn) -> Result<Vec<History>, NoteError> {
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
    fn insert(&self, conn: &DbConn, user: &AuthUser) -> Result<u32, NoteError> {
        use crate::diesel::*;
        use crate::schema::histories::dsl::*;

        user.auth()?;

        diesel::insert_into(histories)
            .values(InsertHistory::from(&*self))
            .execute(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to insert history: {}", err)))?;

        Ok(crate::get_last_insert_rowid(conn)?)
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
            id: history.id,
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

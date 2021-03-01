//! 文章
use crate::auth::{AuthDelete, AuthInsert, AuthUpdate, AuthUser};
use crate::history::History;
use crate::insert::InsertPost;
use crate::{DbConn, NoteError};

pub struct Post {
    id: i32,
    title: String,
    markdown: Option<String>,
}

impl Post {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_markdown(&self) -> &str {
        match &self.markdown {
            Some(markdown) => markdown,
            None => "",
        }
    }
}

impl AuthUpdate for Post {
    fn update(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::posts::dsl::*;

        user.auth()?;

        let history = History::new(self.id, &self.get_markdown());
        history.insert(&*conn, &*user)?;

        diesel::update(posts.filter(id.eq(self.id)))
            .set(InsertPost::from(&*self))
            .execute(conn)
            .map_err(|err| {
                NoteError::SQLError(format!("Failed update post {}: {}", self.id, err))
            })?;

        Ok(())
    }
}

impl AuthInsert for Post {
    fn insert(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::posts;

        user.auth()?;

        diesel::insert_into(posts::table)
            .values(InsertPost::from(&*self))
            .execute(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to insert post: {}", err)))?;

        Ok(())
    }
}

impl AuthDelete for Post {
    fn delete(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::posts::dsl::*;

        user.auth()?;

        let history_list = History::get_history(self.get_id(), &*conn)?;
        for history in history_list {
            history.delete(&*conn, user)?;
        }

        diesel::delete(posts.filter(id.eq(self.id)))
            .execute(conn)
            .map_err(|err| {
                NoteError::SQLError(format!("Failed update post {}: {}", self.id, err))
            })?;

        Ok(())
    }
}

impl From<&Post> for InsertPost {
    fn from(post: &Post) -> InsertPost {
        InsertPost {
            title: String::from(post.get_title()),
            markdown: Some(String::from(post.get_markdown())),
        }
    }
}

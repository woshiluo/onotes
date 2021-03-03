//! 文章
use crate::auth::{AuthDelete, AuthInsert, AuthUpdate, AuthUser};
use crate::history::History;
use crate::insert::InsertPost;
use crate::raw::RawPost;
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

    pub fn new(title: String, markdown: Option<String>) -> Post {
        Post {
            id: 0,
            title,
            markdown,
        }
    }
    pub fn from_id(conn: &DbConn, post_id: i32) -> Result<Post, NoteError> {
        use crate::diesel::*;
        use crate::schema::posts::dsl::*;

        Ok(Post::from(
            &posts
                .filter(id.eq(post_id))
                .first::<RawPost>(conn)
                .map_err(|err| {
                    NoteError::SQLError(format!("Failed to query post from id{}:{}", post_id, err))
                })?,
        ))
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
    fn insert(&self, conn: &DbConn, user: &AuthUser) -> Result<i32, NoteError> {
        use crate::diesel::*;
        use crate::schema::posts;

        user.auth()?;

        diesel::insert_into(posts::table)
            .values(InsertPost::from(&*self))
            .execute(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to insert post: {}", err)))?;

        let return_id = posts::table
            .select(crate::last_insert_rowid)
            .get_result::<i32>(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to query insert id: {}", err)))?;
        Ok(return_id)
    }
}

impl AuthDelete for Post {
    fn delete(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::posts::dsl::*;

        user.auth()?;
        // TODO: Delete all edges from / to this node

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

impl From<&RawPost> for Post {
    fn from(post: &RawPost) -> Post {
        Post {
            id: post.id.expect("Post id is null!"),
            title: post.title.clone(),
            markdown: post.markdown.clone(),
        }
    }
}

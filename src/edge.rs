//! 文章间的上下级关系
use crate::auth::{AuthDelete, AuthInsert, AuthUser};
use crate::insert::InsertEdge;
use crate::raw::RawEdge;
use crate::{DbConn, NoteError};

/// 以存图的方式存放关系
#[derive(Debug)]
pub struct Edge {
    id: i32,
    /// 起点
    from_post: i32,
    /// 终点
    to_post: i32,
}

impl Edge {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_from(&self) -> i32 {
        self.from_post
    }
    pub fn get_to(&self) -> i32 {
        self.to_post
    }

    /// 获取所有起点为 `from_id` 的边
    pub fn get_from_list(&self, conn: &DbConn, from_id: i32) -> Result<Vec<Edge>, NoteError> {
        use crate::diesel::*;
        use crate::schema::post_edge::dsl::*;

        let edge_list = post_edge
            .filter(from_post.eq(from_id))
            .load::<RawEdge>(&conn.0)
            .map_err(|err| {
                NoteError::SQLError(format!("Failed query edge from {}: {}", from_id, err))
            })?
            .iter()
            .map(Edge::from)
            .collect::<Vec<Edge>>();

        Ok(edge_list)
    }
    /// 获取所有终点为 `to_id` 的边
    pub fn get_to_list(&self, conn: &DbConn, to_id: i32) -> Result<Vec<Edge>, NoteError> {
        use crate::diesel::*;
        use crate::schema::post_edge::dsl::*;

        let edge_list = post_edge
            .filter(to_post.eq(to_id))
            .load::<RawEdge>(&conn.0)
            .map_err(|err| NoteError::SQLError(format!("Failed query edge to {}: {}", to_id, err)))?
            .iter()
            .map(Edge::from)
            .collect::<Vec<Edge>>();

        Ok(edge_list)
    }
}

impl AuthInsert for Edge {
    fn insert(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::post_edge;

        user.auth()?;

        diesel::insert_into(post_edge::table)
            .values(InsertEdge::from(&*self))
            .execute(&conn.0)
            .map_err(|err| {
                NoteError::SQLError(format!("Failed to delete edge{:?}: {}", self, err))
            })?;

        Ok(())
    }
}

impl AuthDelete for Edge {
    fn delete(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::post_edge::*;

        user.auth()?;

        diesel::delete(table.filter(id.eq(self.get_id())))
            .execute(&conn.0)
            .map_err(|err| {
                NoteError::SQLError(format!("Failed to delete edge{:?}: {}", self, err))
            })?;

        Ok(())
    }
}

impl From<&RawEdge> for Edge {
    fn from(edge: &RawEdge) -> Edge {
        Edge {
            id: edge.id,
            from_post: edge.from_post,
            to_post: edge.to_post,
        }
    }
}
impl From<&Edge> for InsertEdge {
    fn from(edge: &Edge) -> InsertEdge {
        InsertEdge {
            from_post: edge.from_post,
            to_post: edge.to_post,
        }
    }
}

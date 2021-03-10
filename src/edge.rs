//! 文章间的上下级关系
use crate::auth::{AuthDelete, AuthInsert, AuthUser};
use crate::insert::InsertEdge;
use crate::raw::RawEdge;
use crate::{DbConn, NoteError};

use serde::{Deserialize, Serialize};

/// 以存图的方式存放关系
#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    id: u32,
    /// 起点
    from_post: u32,
    /// 终点
    to_post: u32,
}

impl Edge {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_from(&self) -> u32 {
        self.from_post
    }
    pub fn get_to(&self) -> u32 {
        self.to_post
    }

    pub fn new(from_post: u32, to_post: u32) -> Edge {
        Edge {
            id: 0,
            from_post,
            to_post,
        }
    }

    /// 获取所有起点为 `from_id` 的边
    pub fn get_to_list(conn: &DbConn, from_id: u32) -> Result<Vec<Edge>, NoteError> {
        use crate::diesel::*;
        use crate::schema::post_edge::dsl::*;

        let edge_list = post_edge
            .filter(from_post.eq(from_id))
            .load::<RawEdge>(conn)
            .map_err(|err| {
                NoteError::SQLError(format!("Failed query edge from {}: {}", from_id, err))
            })?
            .iter()
            .map(Edge::from)
            .collect::<Vec<Edge>>();

        Ok(edge_list)
    }
    /// 获取所有起点为 `from_id` 的边
    pub fn update_to_list(
        conn: &DbConn,
        auth: &AuthUser,
        from_id: u32,
        to_list: Vec<&crate::post::Post>,
    ) -> Result<(), NoteError> {
        let origin_to_list = Edge::get_to_list(conn, from_id)?;
        for origin_to in &origin_to_list {
            if !to_list
                .iter()
                .any(|current_to| current_to.get_id() == origin_to.get_to())
            {
                origin_to.delete(&conn, &auth)?;
            }
        }

        for current_to in &to_list {
            if !origin_to_list
                .iter()
                .any(|origin_to| current_to.get_id() == origin_to.get_to())
            {
                Edge::new(from_id, current_to.get_id()).insert(&conn, &auth)?;
            }
        }

        Ok(())
    }
    /// 获取所有终点为 `to_id` 的边
    pub fn get_from_list(conn: &DbConn, to_id: u32) -> Result<Vec<Edge>, NoteError> {
        use crate::diesel::*;
        use crate::schema::post_edge::dsl::*;

        let edge_list = post_edge
            .filter(to_post.eq(to_id))
            .load::<RawEdge>(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed query edge to {}: {}", to_id, err)))?
            .iter()
            .map(Edge::from)
            .collect::<Vec<Edge>>();

        Ok(edge_list)
    }
    pub fn update_from_list(
        conn: &DbConn,
        auth: &AuthUser,
        to_id: u32,
        from_list: Vec<&crate::post::Post>,
    ) -> Result<(), NoteError> {
        let origin_from_list = Edge::get_from_list(conn, to_id)?;
        for origin_from in &origin_from_list {
            if !from_list
                .iter()
                .any(|current_from| current_from.get_id() == origin_from.get_from())
            {
                origin_from.delete(&conn, &auth)?;
            }
        }

        for current_from in &from_list {
            if !origin_from_list
                .iter()
                .any(|origin_from| current_from.get_id() == origin_from.get_from())
            {
                Edge::new(current_from.get_id(), to_id).insert(&conn, &auth)?;
            }
        }

        Ok(())
    }
}

impl AuthInsert for Edge {
    fn insert(&self, conn: &DbConn, user: &AuthUser) -> Result<u32, NoteError> {
        use crate::diesel::*;
        use crate::schema::post_edge;

        user.auth()?;

        diesel::insert_into(post_edge::table)
            .values(InsertEdge::from(&*self))
            .execute(conn)
            .map_err(|err| {
                NoteError::SQLError(format!("Failed to delete edge{:?}: {}", self, err))
            })?;

        Ok(crate::get_last_insert_rowid(conn)?)
    }
}

impl AuthDelete for Edge {
    fn delete(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::post_edge::*;

        user.auth()?;

        diesel::delete(table.filter(id.eq(self.get_id())))
            .execute(conn)
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

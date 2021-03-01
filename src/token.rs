//! 通过 Token 登陆
use crate::auth::{AuthInsert, AuthUser};
use crate::insert::InsertToken;
use crate::raw::RawToken;
use crate::{gen_token, DbConn, NoteError};

pub struct Token {
    user_id: i32,
    token: String,
}

impl Token {
    pub fn new(user_id: i32) -> Token {
        Token {
            user_id,
            token: gen_token(),
        }
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// 获取 Token 等于当前值的列表
    pub fn from_token(current_token: &str, conn: &DbConn) -> Result<Vec<Token>, NoteError> {
        use crate::diesel::*;
        use crate::schema::tokens::dsl::*;

        let raw_list = tokens
            .filter(token.eq(current_token))
            .load::<RawToken>(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to query token: {}", err)))?;

        let mut token_list = vec![];
        for raw_token in raw_list.iter() {
            token_list.push(Token::from(raw_token));
        }

        Ok(token_list)
    }
    /// 验证对应 Token 是否合法
    pub fn verify(id: &i32, token: &str, conn: &DbConn) -> Result<bool, NoteError> {
        let token_list = Token::from_token(token, &*conn)?;

        for token in token_list.iter() {
            if token.get_user_id() == *id {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl AuthInsert for Token {
    fn insert(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        use crate::diesel::*;
        use crate::schema::tokens;

        if user.get_id() != self.user_id {
            NoteError::NoPermission("You can not give other account token".to_string());
        }

        diesel::insert_into(tokens::table)
            .values(InsertToken::from(&*self))
            .execute(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to insert token: {}", err)))?;

        Ok(())
    }
}

impl From<&Token> for InsertToken {
    fn from(token: &Token) -> InsertToken {
        InsertToken {
            user_id: token.get_user_id(),
            token: String::from(token.get_token()),
        }
    }
}

impl From<&RawToken> for Token {
    fn from(raw: &RawToken) -> Token {
        Token {
            user_id: raw.user_id,
            token: String::from(&raw.token),
        }
    }
}

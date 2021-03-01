//! 用户登陆的封装
use crate::token::Token;
use crate::user::User;
use crate::{DbConn, NoteError};

use std::convert::TryFrom;

/// 认证过的用户类型，可以数据库更新
pub struct AuthUser {
    id: i32,
    nickname: String,
    email: String,
    admin: bool,
    level: AuthLevel,
}

/// 用户的登陆方式
#[derive(Clone, Copy)]
pub enum AuthLevel {
    /// 密码
    Password,
    /// Token
    Token,
}

/// 用于登陆的枚举
pub enum Auth {
    Password((String, String)),
    Token((i32, String)),
}

/// 将自身同步进数据库
pub trait AuthUpdate {
    fn update(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError>;
}

/// 将自身插入进数据库
pub trait AuthInsert {
    fn insert(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError>;
}

/// 将自身从数据库中移除
pub trait AuthDelete {
    fn delete(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError>;
}

impl AuthUser {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_nickname(&self) -> &str {
        &self.nickname
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn get_level(&self) -> AuthLevel {
        self.level
    }
    pub fn is_admin(&self) -> bool {
        self.admin
    }

    pub fn auth(&self) -> Result<(), NoteError> {
        match self.is_admin() {
            true => Ok(()),
            false => Err(NoteError::NoPermission(String::from("You are not admin"))),
        }
    }
    /// 增加一个 Token
    pub fn add_token(&self, conn: &DbConn) -> Result<String, NoteError> {
        match self.level {
            AuthLevel::Password => (),
            _ => {
                return Err(NoteError::NoPermission(String::from(
                    "Only password auth can add token",
                )))
            }
        };

        let token = Token::new(self.id);
        token.insert(conn, &*self)?;
        Ok(String::from(token.get_token()))
    }
}

/// 通过 Auth 枚举获得 AuthUser
impl TryFrom<(Auth, &DbConn)> for AuthUser {
    type Error = NoteError;
    fn try_from(item: (Auth, &DbConn)) -> Result<AuthUser, Self::Error> {
        let (auth, conn) = item;

        match &auth {
            Auth::Password((user_name, user_password)) => {
                let user = User::from_nickname(user_name, &*conn).map_err(|err| {
                    NoteError::UserNotFound(format!(
                        "Not found user by nickname\"{}\": {:?}",
                        user_name, err
                    ))
                })?;
                match unsafe { user.verify(user_password)? } {
                    false => Err(NoteError::AuthError("Wrong password".to_string())),
                    true => Ok(AuthUser::from((&user, AuthLevel::Password))),
                }
            }
            Auth::Token((user_id, user_token)) => {
                match Token::verify(user_id, user_token, &*conn)? {
                    false => Err(NoteError::AuthError("Wrong token".to_string())),
                    true => {
                        let user = User::from_user_id(*user_id, &*conn).map_err(|err| {
                            NoteError::UserNotFound(format!(
                                "Not found user by id\"{}\": {:?}",
                                user_id, err
                            ))
                        })?;

                        Ok(AuthUser::from((&user, AuthLevel::Token)))
                    }
                }
            }
        }
    }
}

/// 应当确保这个方法仅在确定验证过后调用
impl From<(&User, AuthLevel)> for AuthUser {
    fn from(item: (&User, AuthLevel)) -> AuthUser {
        let (user, level) = item;
        AuthUser {
            id: user.get_id(),
            nickname: String::from(user.get_nickname()),
            email: String::from(user.get_email()),
            admin: user.is_admin(),
            level,
        }
    }
}

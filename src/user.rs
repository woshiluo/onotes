//! 用户
use crate::auth::{AuthLevel, AuthUpdate, AuthUser};
use crate::insert::InsertUser;
use crate::raw::RawUser;
use crate::{DbConn, NoteError};

/// 用户
pub struct User {
    id: u32,
    nickname: String,
    password: String,
    email: String,
    admin: bool,
}

impl User {
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_nickname(&self) -> &str {
        &self.nickname
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn is_admin(&self) -> bool {
        self.admin
    }

    pub fn new(id: Option<u32>, nickname: String, password: String, email: String) -> User {
        User {
            id: id.unwrap_or(0),
            admin: false,
            nickname,
            password,
            email,
        }
    }

    /// 验证密码是否正确
    /// # Safety
    ///
    /// 无法确认来源是否正确，故使用 unsafe
    /// 当前仅当该 User 是从数据库查询得到的结果时，本函数保证在功能上正确
    pub unsafe fn verify(&self, password: &str) -> Result<bool, NoteError> {
        bcrypt::verify(password, &self.password)
            .map_err(|err| NoteError::AuthError(format!("Failed compare password: {}", err)))
    }

    /// 插入当前用户（很明显，插入用户不需要验证）
    pub fn insert(&mut self, conn: &DbConn) -> Result<u32, NoteError> {
        use crate::diesel::*;
        use crate::schema::users;

        self.admin = false;
        self.password = bcrypt::hash(&self.password, bcrypt::DEFAULT_COST).unwrap();
        diesel::insert_into(users::table)
            .values(InsertUser::from((&*self, self.password.as_str())))
            .execute(conn)
            .map_err(|err| NoteError::SQLError(format!("Failed to insert user: {}", err)))?;

        crate::get_last_insert_rowid(conn)
    }
    /// 通过用户昵称获取用户
    pub fn from_nickname(name: &str, conn: &DbConn) -> Result<User, NoteError> {
        use crate::diesel::*;
        use crate::schema::users::dsl::*;

        Ok(User::from(
            users
                .filter(nickname.eq(name))
                .first::<RawUser>(conn)
                .map_err(|err| {
                    NoteError::SQLError(format!(
                        "Failed to query user from nickname{}: {}",
                        name, err
                    ))
                })?,
        ))
    }
    /// 通过用户 ID 获取用户
    pub fn from_user_id(user_id: u32, conn: &DbConn) -> Result<User, NoteError> {
        use crate::diesel::*;
        use crate::schema::users::dsl::*;

        Ok(User::from(
            users
                .filter(id.eq(user_id))
                .first::<RawUser>(conn)
                .map_err(|err| {
                    NoteError::SQLError(format!("Failed to query user from id{}:{}", user_id, err))
                })?,
        ))
    }
}

impl AuthUpdate for User {
    fn update(&self, conn: &DbConn, user: &AuthUser) -> Result<(), NoteError> {
        match user.get_level() {
            AuthLevel::Password => match user.get_id() == self.id {
                true => {
                    use crate::diesel::*;
                    use crate::schema::users::dsl::*;
                    diesel::update(users.filter(id.eq(self.id)))
                        .set(InsertUser::from((
                            &*self,
                            bcrypt::hash(self.password.as_str(), bcrypt::DEFAULT_COST)
                                .unwrap()
                                .as_str(),
                        )))
                        .execute(conn)
                        .map_err(|err| {
                            NoteError::SQLError(format!("Failed update user {}: {}", self.id, err))
                        })?;
                    Ok(())
                }
                _ => Err(NoteError::NoPermission(String::from(
                    "Only user itself can update user profile",
                ))),
            },
            _ => Err(NoteError::NoPermission(String::from(
                "Only password auth can update user profile",
            ))),
        }
    }
}

impl From<RawUser> for User {
    fn from(raw: RawUser) -> User {
        User {
            id: raw.id,
            nickname: raw.nickname,
            password: raw.password,
            email: raw.email,
            admin: raw.admin,
        }
    }
}

impl From<(&User, &str)> for InsertUser {
    fn from(item: (&User, &str)) -> InsertUser {
        let (user, password) = item;
        InsertUser {
            nickname: String::from(user.get_nickname()),
            email: String::from(user.get_email()),
            password: String::from(password),
        }
    }
}

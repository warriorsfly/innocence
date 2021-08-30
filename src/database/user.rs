use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    errors::Error,
    jwt::{create_jwt, hash, Claims},
    schema::users,
};

use super::Connection;

#[derive(Debug, Deserialize, Queryable, Identifiable, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub bio: String,
    pub avatar: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub bio: &'a str,
    pub avatar: &'a str,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct NewUserInput {
    pub username: String,
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserLoginInput {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserLoginOutput {
    pub token: String,
    pub account: User,
}

pub(crate) fn signup<'a>(conn: &'a mut Connection, entity: &'a NewUser) -> Result<User, Error> {
    use crate::schema::users::dsl::*;

    diesel::insert_into(users)
        .values(entity)
        .get_result::<User>(conn)
        .map_err(|err| Error::DataBaseError(err.to_string()))
}

pub(crate) fn login<'a>(
    conn: &'a mut Connection,
    name: &'a str,
    psw: &'a str,
) -> Result<UserLoginOutput, Error> {
    use crate::schema::users::dsl::*;
    let hashed = hash(psw);

    let user: User = users
        .filter(username.eq(name))
        .filter(password.eq(&hashed))
        .get_result(conn)?;
    let claims = Claims::new(user.id);
    let user_login_output = UserLoginOutput {
        token: create_jwt(claims)?,
        account: user,
    };
    Ok(user_login_output)
}
